---
name: chrome-recon
description: "Browser-based API reverse engineering using Chrome DevTools MCP. Use when you need to navigate a web app, capture network traffic, save HAR files, and analyze backend API patterns. Runs as a sub-agent to avoid polluting the main context window with verbose browser output."
allowed-tools: Agent, Bash, Read, Write, Glob, Grep
user-invocable: true
version: 1.0.0
---

# Chrome Recon — Browser API Reverse Engineering

This skill delegates all Chrome DevTools MCP interaction to a sub-agent, keeping the main context window clean. It captures network traffic progressively as HAR files for deterministic post-analysis.

## Prerequisites

The project must have the Chrome DevTools MCP server configured:
- `.mcp.json` — registers the `chromeDevTools` MCP server
- `scripts/chrome-devtools-mcp-wrapper.sh` — self-contained nix-shell script providing the MCP server + Chromium

Copies of both files are stored in this skill folder for bootstrap reference.

## How to Use

When invoked, launch a **sub-agent** with `subagent_type: "general-purpose"` to do all Chrome DevTools MCP interaction. The sub-agent handles the verbose browser output and returns only the essential findings.

### Sub-Agent Prompt Template

Adapt this template based on the user's target:

```
You have access to Chrome DevTools MCP tools. Your task is to reverse-engineer the backend API of a web application.

TARGET: {url_or_description}
GOAL: {what_to_discover}

## Workflow

1. **Start network recording immediately**
   - Use `mcp__chromeDevTools__navigate_page` to open the target URL
   - Network recording begins automatically on page load

2. **Interact with the application**
   - Navigate through the workflows the user specified
   - Use click, fill, type_text tools as needed
   - Take screenshots at key points for context

3. **Capture network traffic at each major step**
   After each logical workflow step (login, page navigation, form submission, etc.):
   - Use `mcp__chromeDevTools__list_network_requests` to get all captured requests
   - Use `mcp__chromeDevTools__get_network_request` for interesting requests (API calls, not static assets)
   - Filter for XHR/fetch requests — ignore images, CSS, fonts, static JS

4. **Save HAR files progressively**
   At each major workflow step, save a HAR file:
   - Use `mcp__chromeDevTools__list_network_requests` to collect all requests since last save
   - For each relevant request, use `mcp__chromeDevTools__get_network_request` to get full details (headers, body, response)
   - Write a HAR-formatted JSON file to: `har/{timestamp}_{step_name}.har`
   - HAR spec: http://www.softwareishard.com/blog/har-12-spec/

5. **Report findings**
   Return a structured summary:
   - Base URL and API prefix patterns
   - Authentication mechanism (cookies, bearer tokens, API keys, etc.)
   - Discovered endpoints with methods, paths, request/response shapes
   - Any pagination, rate limiting, or versioning patterns
   - Interesting headers (custom headers, CORS, caching)
```

### HAR File Structure

Each HAR file should follow the HAR 1.2 spec. Minimal structure:

```json
{
  "log": {
    "version": "1.2",
    "creator": { "name": "chrome-recon", "version": "1.0.0" },
    "entries": [
      {
        "startedDateTime": "...",
        "request": {
          "method": "POST",
          "url": "https://...",
          "headers": [...],
          "postData": { "mimeType": "application/json", "text": "..." }
        },
        "response": {
          "status": 200,
          "headers": [...],
          "content": { "mimeType": "application/json", "text": "..." }
        }
      }
    ]
  }
}
```

### HAR File Naming Convention

```
har/
  001_initial_load.har
  002_login.har
  003_dashboard_navigation.har
  004_api_exploration.har
```

Prefix with zero-padded sequence number for sort order. Use descriptive step names.

## Output Directory

HAR files go in `har/` at the project root. This directory:
- Contains PII (auth tokens, session cookies, user data)
- Must be in `.gitignore`
- Is the primary artifact for post-analysis

## Post-Analysis

After the sub-agent completes, HAR files can be analyzed:
- Load into browser DevTools (Network tab → Import HAR)
- Parse with `jq` for endpoint cataloging
- Diff HAR files to identify state changes between steps
- Extract API schemas from request/response pairs

## Example Invocation

User says: "reverse engineer the API for https://example.com/app"

You should:
1. Ensure `har/` directory exists and is in `.gitignore`
2. Launch sub-agent with the Chrome DevTools MCP tools
3. Sub-agent navigates, captures, saves HAR files
4. Sub-agent returns API summary
5. You present the findings to the user

## Important Notes

- HAR files contain sensitive data (cookies, tokens, PII) — never commit them
- The sub-agent should filter out static asset requests (images, CSS, fonts, JS bundles)
- Focus on XHR/fetch requests to API endpoints
- If the app requires login, the user must provide credentials or handle auth manually
- Take screenshots sparingly — they consume context. Prefer network data.
