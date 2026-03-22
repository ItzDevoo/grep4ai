#!/usr/bin/env node

"use strict";

const message = `
┌─────────────────────────────────────────────────────────┐
│                                                         │
│   ✓ grep4ai installed successfully                      │
│                                                         │
│   Want AI agents to use it? One command:                │
│                                                         │
│     claude mcp add grep4ai -- npx @grep4ai/mcp          │
│                                                         │
│   That's it. No config files, no restart needed.        │
│                                                         │
│   For Cursor / other MCP clients, add to your config:   │
│                                                         │
│     {                                                   │
│       "mcpServers": {                                   │
│         "grep4ai": {                                    │
│           "command": "npx",                             │
│           "args": ["@grep4ai/mcp"]                      │
│         }                                               │
│       }                                                 │
│     }                                                   │
│                                                         │
│   Docs: https://github.com/ItzDevoo/grep4ai             │
│                                                         │
└─────────────────────────────────────────────────────────┘
`;

console.log(message);
