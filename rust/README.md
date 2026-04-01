# Claw Code - Rust Implementation

A high-performance Rust rewrite of the Claw Code CLI agent harness. Built for speed, safety, and native tool execution.

This Rust workspace is part of a repository that began from a clone of:

`https://github.com/instructkr/claw-code`

and was then modified and cleaned up independently.

## Quick Start

```bash
# Build
cd rust/
cargo build --release

# Run interactive REPL
./target/release/claw

# One-shot prompt
./target/release/claw prompt "explain this codebase"

# With specific model
./target/release/claw --model sonnet prompt "fix the bug in main.rs"
```

## Configuration

Set your API credentials:

```bash
export ANTHROPIC_API_KEY="sk-ant-..."
# Or use a proxy
export ANTHROPIC_BASE_URL="https://your-proxy.com"
```

OpenAI-compatible providers are also supported:

```bash
# OpenAI
export OPENAI_API_KEY="sk-..."

# xAI
export XAI_API_KEY="xai-..."

# Groq
export GROQ_API_KEY="gsk-..."

# Google Gemini
export GEMINI_API_KEY="AIza..."

# Hugging Face router
export HF_TOKEN="hf_..."
```

You can persist provider defaults in `.claw/settings.json` or
`.claw/settings.local.json`:

```json
{
  "model": "openai/gpt-oss-120b",
  "env": {
    "GROQ_API_KEY": "gsk-..."
  },
  "providers": {
    "groq": {
      "baseUrl": "https://api.groq.com/openai/v1"
    }
  }
}
```

Examples:

```bash
# Groq GPT OSS
export GROQ_API_KEY="gsk-..."
./target/release/claw --model openai/gpt-oss-120b --base-url https://api.groq.com/openai/v1

# Groq GPT OSS 20B
./target/release/claw --model openai/gpt-oss-20b --base-url https://api.groq.com/openai/v1

# Gemini Flash via Google's OpenAI-compatible endpoint
export GEMINI_API_KEY="AIza..."
./target/release/claw --model gemini-3-flash-preview --base-url https://generativelanguage.googleapis.com/v1beta/openai

# Hugging Face router
export HF_TOKEN="hf_..."
./target/release/claw --model Qwen/Qwen2.5-Coder-32B-Instruct --base-url https://router.huggingface.co/v1
```

Or authenticate via OAuth:

```bash
claw login
```

## Features

| Feature | Status |
|---------|--------|
| API + streaming | ✅ |
| OAuth login/logout | ✅ |
| Interactive REPL (rustyline) | ✅ |
| Tool system (bash, read, write, edit, grep, glob) | ✅ |
| Web tools (search, fetch) | ✅ |
| Sub-agent orchestration | ✅ |
| Todo tracking | ✅ |
| Notebook editing | ✅ |
| CLAW.md / project memory | ✅ |
| Config file hierarchy (.claw.json) | ✅ |
| Permission system | ✅ |
| MCP server lifecycle | ✅ |
| Session persistence + resume | ✅ |
| Extended thinking (thinking blocks) | ✅ |
| Cost tracking + usage display | ✅ |
| Git integration | ✅ |
| Markdown terminal rendering (ANSI) | ✅ |
| Model aliases (opus/sonnet/haiku) | ✅ |
| Slash commands (/status, /compact, /clear, etc.) | ✅ |
| Hooks (PreToolUse/PostToolUse) | 🔧 Config only |
| Plugin system | 📋 Planned |
| Skills registry | 📋 Planned |

## Model Aliases

Short names resolve to the latest model versions:

| Alias | Resolves To |
|-------|------------|
| `opus` | `claude-opus-4-6` |
| `sonnet` | `claude-sonnet-4-6` |
| `haiku` | `claude-haiku-4-5-20251213` |

## CLI Flags

```
claw [OPTIONS] [COMMAND]

Options:
  --model MODEL                    Set the model (alias or full name)
  --base-url URL                   Override the API base URL for the active model/provider
  --dangerously-skip-permissions   Skip all permission checks
  --permission-mode MODE           Set read-only, workspace-write, or danger-full-access
  --allowedTools TOOLS             Restrict enabled tools
  --output-format FORMAT           Output format (text or json)
  --version, -V                    Print version info

Commands:
  prompt <text>      One-shot prompt (non-interactive)
  login              Authenticate via OAuth
  logout             Clear stored credentials
  init               Initialize project config
  doctor             Check environment health
  self-update        Update to latest version
```

## Slash Commands (REPL)

| Command | Description |
|---------|-------------|
| `/help` | Show help |
| `/status` | Show session status (model, tokens, cost) |
| `/cost` | Show cost breakdown |
| `/compact` | Compact conversation history |
| `/clear` | Clear conversation |
| `/model [name]` | Show or switch model |
| `/models` | Manage models, API keys, and base URLs |
| `/permissions` | Show or switch permission mode |
| `/config [section]` | Show config (env, hooks, model, plugins, providers) |
| `/memory` | Show CLAW.md contents |
| `/diff` | Show git diff |
| `/export [path]` | Export conversation |
| `/session [id]` | Resume a previous session |
| `/version` | Show version |

## Workspace Layout

```
rust/
├── Cargo.toml              # Workspace root
├── Cargo.lock
└── crates/
    ├── api/                # API client + SSE streaming
    ├── commands/           # Shared slash-command registry
    ├── compat-harness/     # TS manifest extraction harness
    ├── runtime/            # Session, config, permissions, MCP, prompts
    ├── claw-cli/   # Main CLI binary (`claw`)
    └── tools/              # Built-in tool implementations
```

### Crate Responsibilities

- **api** — HTTP client, SSE stream parser, request/response types, auth (API key + OAuth bearer)
- **commands** — Slash command definitions and help text generation
- **compat-harness** — Compatibility and manifest extraction utilities used by this workspace
- **runtime** — `ConversationRuntime` agentic loop, `ConfigLoader` hierarchy, `Session` persistence, permission policy, MCP client, system prompt assembly, usage tracking
- **claw-cli** — REPL, one-shot prompt, streaming display, tool call rendering, CLI argument parsing
- **tools** — Tool specs + execution: Bash, ReadFile, WriteFile, EditFile, GlobSearch, GrepSearch, WebSearch, WebFetch, Agent, TodoWrite, NotebookEdit, Skill, ToolSearch, REPL runtimes

## Stats

- **~20K lines** of Rust
- **6 crates** in workspace
- **Binary name:** `claw`
- **Default model:** `claude-opus-4-6`
- **Default permissions:** `danger-full-access`

## License

See repository root.
