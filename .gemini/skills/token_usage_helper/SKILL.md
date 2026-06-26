---
name: token-usage-helper
description: Use when the user asks about AI token usage, cost, spend, quotas, model/client/workspace/project/session breakdowns, daily/hourly/weekly/monthly reports, pricing, or local AI coding-agent usage data. This skill guides Gemini to answer token and money questions with the installed `tokscale` CLI, including reports by exact date range, project/workspace, session, client, provider, model, and subscription usage.
---

# Token Usage Helper

Use the installed `tokscale` CLI as the source of truth for local AI coding-agent token and cost usage. Answer concretely with exact date ranges, totals, costs, and the breakdown the user asked for.

## Core Rules

- Prefer `tokscale ... --json --no-spinner` for machine-readable output.
- Use `tokscale ... --light --no-spinner` only when the user wants a readable table or when JSON is not needed.
- Always state the exact date range used.
- Treat `--week` as "last 7 days", not "current calendar week".
- For "this week", compute the local calendar-week dates and use `--since YYYY-MM-DD --until YYYY-MM-DD`.
- Date filters are inclusive and use the local timezone.
- Trust the installed binary's `--help` over README text. Local docs may mention commands that are not present in the installed version.
- Keep final answers concise: totals first, then the requested breakdown, then caveats.

## First Checks

Run these before deeper or ambiguous analysis:

```bash
tokscale --version
tokscale --help
tokscale clients --json
```

Use `tokscale clients --json` to learn which clients have local data, message counts, scan paths, missing paths, and diagnostics.

## Common Report Commands

Overall usage and cost:

```bash
tokscale --json --no-spinner
tokscale --light --no-spinner
```

By model, client, or provider:

```bash
tokscale models --json --no-spinner
tokscale models --light --no-spinner
tokscale models --json --group-by model --no-spinner
tokscale models --json --group-by client,model --no-spinner
tokscale models --json --group-by client,provider,model --no-spinner
```

By workspace/project:

```bash
tokscale models --json --group-by workspace,model --no-spinner
tokscale models --light --group-by workspace,model --no-spinner
```

By session:

```bash
tokscale models --json --group-by session,model --no-spinner
tokscale models --light --group-by session,model --no-spinner
tokscale models --json --group-by client,session,model --no-spinner
tokscale --session <SESSION_ID> --json --no-spinner
```

By day or hour:

```bash
tokscale monthly --json --no-spinner
tokscale monthly --light --no-spinner
tokscale hourly --json --no-spinner
tokscale hourly --light --no-spinner
```

Session time metrics:

```bash
tokscale time-metrics --json --no-spinner
```

Subscription/provider quota usage:

```bash
tokscale usage --json
tokscale usage --light
```

Contribution graph export:

```bash
tokscale graph --output data.json --no-spinner
```

## Filters

Date filters:

```bash
tokscale --today --json --no-spinner
tokscale --yesterday --json --no-spinner
tokscale --week --json --no-spinner
tokscale --month --json --no-spinner
tokscale --since 2026-06-22 --until 2026-06-26 --json --no-spinner
tokscale --year 2026 --json --no-spinner
```

Client filters are repeatable or comma-separated:

```bash
tokscale --client codex --json --no-spinner
tokscale --client claude,codex,gemini --json --no-spinner
tokscale -c claude -c codex --json --no-spinner
```

Combined examples:

```bash
tokscale models --client codex --since 2026-06-22 --until 2026-06-26 --group-by session,model --json --no-spinner
tokscale hourly --client claude,codex --today --json --no-spinner
tokscale models --client codex --yesterday --group-by workspace,model --json --no-spinner
```

## Pricing

Look up model pricing:

```bash
tokscale pricing gpt-5.5 --json --no-spinner
tokscale pricing claude-opus-4-8 --json --no-spinner
tokscale pricing list-overrides --json --no-spinner
```

If costs look wrong for a new or custom model, mention that `tokscale` supports custom pricing overrides in `~/.config/tokscale/custom-pricing.json`. Do not edit pricing overrides unless the user explicitly asks.

## Integrations and Cache-Based Clients

Some clients require sync or authentication before reports include fresh data:

```bash
tokscale cursor status
tokscale cursor sync --json
tokscale antigravity status
tokscale antigravity sync
tokscale trae status
tokscale trae sync --since 30
tokscale warp status
tokscale warp sync
tokscale codex accounts --json
tokscale codex status --json
```

Do not run login, sync, submit, logout, delete, switch, remove, or browser-opening commands unless necessary for the user's request. State what account, external service, or local cache will be touched before doing so.

## Headless Codex Capture

For Codex CLI commands whose JSON output would otherwise not be stored in normal sessions, wrap the command:

```bash
tokscale headless codex exec -m gpt-5 "review code changes"
```

Manual capture is possible by saving Codex JSONL output under `~/.config/tokscale/headless/codex/`, but prefer the wrapper when available.

## Output Interpretation

For `models --json`, expect fields like:

- `input`
- `output`
- `cacheRead`
- `cacheWrite`
- `reasoning`
- `messageCount`
- `cost`
- `totalInput`
- `totalOutput`
- `totalCacheRead`
- `totalCacheWrite`
- `totalMessages`
- `totalCost`

The displayed total token count in `--light` reports is normally:

```text
input + output + cacheRead + cacheWrite
```

Reasoning tokens may be present as a separate field. Call this out when relevant instead of silently adding it to the displayed total.

When summarizing, include:

- Exact date range and timezone assumption when relevant.
- Total tokens and total cost.
- Message count.
- The requested breakdown, usually by client, model, workspace/project, or session.
- Caveats from diagnostics, missing scan paths, sync-required clients, or vendor-reported quotas/costs.

For large reports, show top contributors and totals rather than dumping full JSON.
