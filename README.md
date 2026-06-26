# Tokscale

Tokscale is a CLI tool for analyzing local token, cost, and session data from AI coding agents. This repository also includes agent skills so Codex, Claude, Gemini, or OpenCode can automatically choose the right `tokscale` commands.

## Quick Start

Check whether the CLI is installed:

```bash
tokscale --version
tokscale --help
```

If `tokscale` is not installed yet:

```bash
npm install -g tokscale
```

Alternatively, run it without a global install:

```bash
npx tokscale@latest --help
bunx tokscale@latest --help
```

## Using The Skills

Open this repository with your agent and ask about token or cost usage. Skill files are included for multiple agents:

- Codex: `.codex/skills/token-usage-helper/`
- Claude: `.claude/skills/token_usage_helper/`
- Gemini: `.gemini/skills/token_usage_helper/`
- OpenCode: `.opencode/skills/token_usage_helper/`

Examples:

```text
Use $token-usage-helper: How many tokens did I use this week?
Use $token-usage-helper: Show my costs by project and model.
Use $token-usage-helper: Which sessions were most expensive yesterday?
Use $token-usage-helper: Compare Codex and Claude for this month.
```

The skill uses the locally installed `tokscale` CLI as the source of truth. If this README and the CLI help output disagree, trust `tokscale --help`.

## Useful Commands

Diagnose local data sources:

```bash
tokscale clients --json
```

Overall summary:

```bash
tokscale --light --no-spinner
tokscale --json --no-spinner
```

Breakdowns by model, client, workspace, or session:

```bash
tokscale models --light --no-spinner
tokscale models --json --group-by client,model --no-spinner
tokscale models --json --group-by workspace,model --no-spinner
tokscale models --json --group-by session,model --no-spinner
```

Date filters:

```bash
tokscale --today --json --no-spinner
tokscale --yesterday --json --no-spinner
tokscale --week --json --no-spinner
tokscale --month --json --no-spinner
tokscale --since 2026-06-01 --until 2026-06-30 --json --no-spinner
```

Daily and hourly reports:

```bash
tokscale monthly --light --no-spinner
tokscale hourly --light --no-spinner
```

Check pricing:

```bash
tokscale pricing gpt-5 --json --no-spinner
tokscale pricing list-overrides --json --no-spinner
```

## Notes For Agents

- Always use `--no-spinner` for automated commands.
- Prefer `--json --no-spinner` for machine-readable output.
- `--week` means the last 7 days, not the current calendar week.
- For "this week", use an explicit range with `--since YYYY-MM-DD --until YYYY-MM-DD`.
- Date filters are inclusive and use the local timezone.
- Only run login, sync, submit, delete, or browser-opening commands when the user explicitly needs them.

## Privacy

Tokscale reads local usage data from installed AI coding agents. Normal report commands such as `tokscale --json` or `tokscale models --json` analyze local files.

Some integrations may require additional sync or login steps, for example Cursor, Trae, Warp, or Antigravity. These commands may touch external accounts or local caches and should be run deliberately.

## Development

Install dependencies:

```bash
bun install
```

Build the CLI locally:

```bash
bun run build
```

Run the CLI from this repository:

```bash
bun run cli -- --help
```

Run tests for the Rust CLI:

```bash
cargo test -p tokscale-cli
```

## License

See [LICENSE](LICENSE).
