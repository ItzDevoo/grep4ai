#!/usr/bin/env node

/**
 * grep4ai MCP Server
 *
 * Exposes grep4ai as a native tool for AI agents via the
 * Model Context Protocol. One tool call replaces:
 *   grep → parse → rank → context → budget
 *
 * Usage:
 *   npx grep4ai-mcp          # stdio transport (Claude Code, Cursor, etc.)
 *
 * Claude Code config (~/.claude/settings.json):
 *   {
 *     "mcpServers": {
 *       "grep4ai": {
 *         "command": "node",
 *         "args": ["/path/to/grep4ai/mcp/index.js"]
 *       }
 *     }
 *   }
 */

"use strict";

const { McpServer } = require("@modelcontextprotocol/sdk/server/mcp.js");
const {
  StdioServerTransport,
} = require("@modelcontextprotocol/sdk/server/stdio.js");
const { execFile } = require("child_process");
const path = require("path");
const os = require("os");
const { z } = require("zod");

// ── Resolve the grep4ai binary ─────────────────────────────────────

function findBinary() {
  // 1. Check if grep4ai is in PATH
  const name = os.platform() === "win32" ? "grep4ai.exe" : "grep4ai";

  // 2. Check common install locations
  const candidates = [
    // Bundled via @grep4ai/cli dependency (npx / node_modules)
    path.join(__dirname, "node_modules", "@grep4ai", "cli", "bin", name),
    path.join(__dirname, "..", "..", "@grep4ai", "cli", "bin", name),
    // npm global install
    path.join(
      process.env.APPDATA || "",
      "npm",
      "node_modules",
      "@grep4ai",
      "cli",
      "bin",
      name
    ),
    // Legacy unscoped npm global
    path.join(
      process.env.APPDATA || "",
      "npm",
      "node_modules",
      "grep4ai",
      "bin",
      name
    ),
    // cargo install
    path.join(os.homedir(), ".cargo", "bin", name),
    // Local development (relative to this file)
    path.join(__dirname, "..", "target", "release", name),
  ];

  // Try PATH first via 'which' equivalent
  const { execSync } = require("child_process");
  try {
    const found = execSync(
      os.platform() === "win32" ? `where ${name}` : `which ${name}`,
      { encoding: "utf8" }
    ).trim();
    if (found) return found.split("\n")[0].trim();
  } catch {
    // not in PATH
  }

  // Try candidates
  const fs = require("fs");
  for (const candidate of candidates) {
    if (fs.existsSync(candidate)) return candidate;
  }

  return name; // fallback to PATH lookup at exec time
}

const GREP4AI_BIN = findBinary();

// ── Validate binary at startup ────────────────────────────────────

function validateBinary() {
  const { execSync } = require("child_process");
  try {
    const versionOutput = execSync(`"${GREP4AI_BIN}" --version`, {
      encoding: "utf8",
      timeout: 5000,
    }).trim();
    // Extract version string (e.g., "grep4ai 0.4.0" -> "0.4.0")
    const match = versionOutput.match(/(\d+\.\d+\.\d+)/);
    if (match) {
      return match[1];
    }
    return versionOutput;
  } catch (error) {
    console.error(
      `grep4ai-mcp: FATAL — grep4ai binary not found or not executable.\n` +
      `  Looked for: ${GREP4AI_BIN}\n` +
      `  Install with: npm install -g grep4ai\n` +
      `  Or build from source: cargo install --path crates/core\n` +
      `  Error: ${error.message}`
    );
    process.exit(1);
  }
}

const GREP4AI_VERSION = validateBinary();

// ── Execute grep4ai ────────────────────────────────────────────────

const TIMEOUT_MS = 10000; // 10 seconds

function runGrep4ai(args) {
  return new Promise((resolve, reject) => {
    const child = execFile(
      GREP4AI_BIN,
      args,
      {
        maxBuffer: 50 * 1024 * 1024, // 50MB
        timeout: TIMEOUT_MS,
        env: process.env,
      },
      (error, stdout, stderr) => {
        if (error) {
          // Timeout detection
          if (error.killed || error.signal === "SIGTERM") {
            reject(new Error(JSON.stringify({
              error: "timeout",
              suggestion: "try a more specific pattern or path"
            })));
            return;
          }
          // Exit code 1 = no matches (grep convention) — still valid output
          if (error.code === 1 && stdout) {
            resolve(stdout);
          } else if (stdout) {
            // Other errors but we got output — return it
            resolve(stdout);
          } else {
            reject(new Error(`grep4ai failed: ${stderr || error.message}`));
          }
        } else {
          resolve(stdout);
        }
      }
    );
  });
}

// ── Input validation ──────────────────────────────────────────────

function validateSearchInput(pattern, paths) {
  if (!pattern || pattern.trim().length === 0) {
    throw new Error("pattern must not be empty");
  }
  if (pattern.length > 500) {
    throw new Error("pattern must be 500 characters or fewer");
  }
  if (paths) {
    for (const p of paths) {
      if (p.includes("../") || p.includes("..\\")) {
        throw new Error(`path traversal not allowed: ${p}`);
      }
    }
  }
}

// ── MCP Server ─────────────────────────────────────────────────────

const server = new McpServer({
  name: "grep4ai",
  version: GREP4AI_VERSION,
});

// Main search tool
server.tool(
  "search",
  "Search for a pattern in files with AI-optimized ranked results. Returns structured JSON with relevance scoring, context windows, and optional token budget enforcement. Much faster and more useful than raw grep for code understanding.",
  {
    pattern: z.string().describe("Regex pattern to search for (or literal string with fixed_string=true)"),
    paths: z.array(z.string()).optional().describe("Files or directories to search (default: current directory)"),
    ignore_case: z.boolean().optional().describe("Case-insensitive search"),
    word: z.boolean().optional().describe("Match whole words only"),
    fixed_string: z.boolean().optional().describe("Treat pattern as literal string, not regex"),
    file_type: z.array(z.string()).optional().describe("Only search files of this type (e.g., 'rust', 'python', 'js', 'ts')"),
    glob: z.array(z.string()).optional().describe("Include files matching glob patterns (e.g., '*.tsx')"),
    context: z.number().optional().describe("Lines of context around each match (default: 2)"),
    token_budget: z.number().optional().describe("Maximum tokens in output — enforces budget with greedy packing"),
    max_results: z.number().optional().describe("Maximum number of results to return (default: 100)"),
    dedup: z.boolean().optional().describe("Collapse near-duplicate results"),
    no_rank: z.boolean().optional().describe("Disable relevance ranking"),
    explain: z.boolean().optional().describe("Include signal breakdown explaining why each result ranked where it did"),
    hidden: z.boolean().optional().describe("Search hidden files and directories"),
    no_ignore: z.boolean().optional().describe("Don't respect .gitignore files"),
    max_depth: z.number().optional().describe("Maximum directory traversal depth"),
    max_filesize: z.string().optional().describe("Skip files larger than this (e.g., '1M', '500K')"),
  },
  async ({ pattern, paths, ignore_case, word, fixed_string, file_type, glob,
           context, token_budget, max_results, dedup, no_rank, explain,
           hidden, no_ignore, max_depth, max_filesize }) => {
    try {
      validateSearchInput(pattern, paths);
    } catch (error) {
      return {
        content: [{ type: "text", text: `Validation error: ${error.message}` }],
        isError: true,
      };
    }

    const args = [];

    // Flags
    if (ignore_case) args.push("-i");
    if (word) args.push("-w");
    if (fixed_string) args.push("-F");
    if (dedup) args.push("--dedup");
    if (no_rank) args.push("--no-rank");
    if (explain) args.push("--explain");
    if (hidden) args.push("--hidden");
    if (no_ignore) args.push("--no-ignore");

    // Value options
    if (context != null) args.push("-C", String(context));
    if (token_budget != null) args.push("--token-budget", String(token_budget));
    if (max_results != null) args.push("--max-results", String(max_results));
    if (max_depth != null) args.push("--max-depth", String(max_depth));
    if (max_filesize) args.push("--max-filesize", max_filesize);

    // Multi-value options
    if (file_type) {
      for (const t of file_type) args.push("-t", t);
    }
    if (glob) {
      for (const g of glob) args.push("-g", g);
    }

    // Always JSON output
    args.push("-f", "json");

    // Pattern (required)
    args.push(pattern);

    // Paths
    if (paths && paths.length > 0) {
      args.push(...paths);
    }

    try {
      const output = await runGrep4ai(args);
      return {
        content: [{ type: "text", text: output }],
      };
    } catch (error) {
      return {
        content: [{ type: "text", text: `Error: ${error.message}` }],
        isError: true,
      };
    }
  }
);

// Quick find definitions tool — convenience wrapper
server.tool(
  "find_definitions",
  "Find definitions (functions, classes, structs, types) matching a symbol name. Pass the actual identifier, not a description — e.g., 'BudgetEnforcer' not 'token budget'. Uses regex to match common definition keywords followed by the name.",
  {
    name: z.string().describe("The actual symbol/identifier name to find (e.g., 'BudgetEnforcer', 'authenticate', 'UserConfig'). Must be the code identifier, not a concept description."),
    paths: z.array(z.string()).optional().describe("Directories to search (default: current directory)"),
    file_type: z.array(z.string()).optional().describe("Limit to specific file types"),
    token_budget: z.number().optional().describe("Maximum tokens in output"),
  },
  async ({ name, paths, file_type, token_budget }) => {
    try {
      validateSearchInput(name, paths);
    } catch (error) {
      return {
        content: [{ type: "text", text: `Validation error: ${error.message}` }],
        isError: true,
      };
    }

    // Escape special regex characters in the name to prevent regex injection
    const escapedName = name.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    // Build a pattern that matches common definition forms
    const pattern = `(fn |def |function |class |struct |enum |trait |interface |type |const |export ).*${escapedName}`;

    const args = ["-f", "json", "--explain"];

    if (token_budget != null) args.push("--token-budget", String(token_budget));
    if (file_type) {
      for (const t of file_type) args.push("-t", t);
    }

    args.push(pattern);

    if (paths && paths.length > 0) {
      args.push(...paths);
    }

    try {
      const output = await runGrep4ai(args);
      return {
        content: [{ type: "text", text: output }],
      };
    } catch (error) {
      return {
        content: [{ type: "text", text: `Error: ${error.message}` }],
        isError: true,
      };
    }
  }
);

// Health check tool
server.tool(
  "ping",
  "Health check — verifies the grep4ai binary is reachable and returns its version.",
  {},
  async () => {
    try {
      const { execSync } = require("child_process");
      const versionOutput = execSync(`"${GREP4AI_BIN}" --version`, {
        encoding: "utf8",
        timeout: 5000,
      }).trim();

      return {
        content: [{
          type: "text",
          text: JSON.stringify({
            status: "ok",
            binary: GREP4AI_BIN,
            version: GREP4AI_VERSION,
            raw_version: versionOutput,
          }, null, 2),
        }],
      };
    } catch (error) {
      return {
        content: [{
          type: "text",
          text: JSON.stringify({
            status: "error",
            binary: GREP4AI_BIN,
            error: error.message,
          }, null, 2),
        }],
        isError: true,
      };
    }
  }
);

// ── Start server ───────────────────────────────────────────────────

async function main() {
  const transport = new StdioServerTransport();
  await server.connect(transport);
  // Server is now running on stdio
}

main().catch((error) => {
  console.error("grep4ai-mcp: fatal error:", error);
  process.exit(1);
});
