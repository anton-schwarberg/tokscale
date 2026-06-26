---
name: token-usage-helper
description: Use when the user asks about AI token usage, cost, spend, quotas, model/client/workspace/project/session breakdowns, weekly/monthly/daily/hourly usage, pricing, or local AI coding-agent usage data. This skill guides Codex to answer token and money questions with the installed `tokscale` CLI, including reports by project/workspace, session, client, provider, model, date range, and subscription usage.
---

# Token Usage Helper

## Overview

Use the installed `tokscale` CLI to answer token and cost questions from local AI-agent usage data. Prefer commands with `--json --no-spinner` for machine-readable totals, and use `--light --no-spinner` when the user wants a readable table.

Always state the exact date range used. `--week` means the last 7 days, not the current calendar week. For "this week", compute the local calendar-week dates and use `--since YYYY-MM-DD --until YYYY-MM-DD`.

## First Checks

Run these before deeper analysis:

```bash
tokscale --version
tokscale --help
tokscale clients --json
```

Use `tokscale clients --json` to see which clients have local data and where `tokscale` is scanning. If a README exists, treat it as secondary to the installed binary's `--help`; local docs can describe commands not present in the installed version.

## Common Reports

Overall usage and cost:

```bash
tokscale --json --no-spinner
tokscale --light --no-spinner
```

By model/client/provider:

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

Date filters are inclusive and use local timezone:

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

Combine filters freely:

```bash
tokscale models --client codex --since 2026-06-22 --until 2026-06-26 --group-by session,model --json --no-spinner
tokscale hourly --client claude,codex --today --json --no-spinner
```

## Pricing

Look up model pricing:

```bash
tokscale pricing gpt-5.5 --json --no-spinner
tokscale pricing claude-opus-4-8 --json --no-spinner
tokscale pricing list-overrides --json --no-spinner
```

If costs look wrong for a new or custom model, mention that `tokscale` can use custom pricing overrides in `~/.config/tokscale/custom-pricing.json`. Do not edit pricing overrides unless the user asks.

## Headless Codex Capture

For Codex CLI commands whose JSON output would otherwise not be stored in normal sessions, wrap the command:

```bash
tokscale headless codex exec -m gpt-5 "review code changes"
```

Manual capture is also possible by saving Codex JSONL output under `~/.config/tokscale/headless/codex/`, but prefer the wrapper when available.

## Integrations

Some clients require explicit sync or authentication before reports include them:

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

Do not run login, sync, submit, logout, delete, switch, remove, or browser-opening commands unless they are needed for the user's request. Explain what external account or local cache will be touched first.

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

The displayed total token count in light reports is normally `input + output + cacheRead + cacheWrite`. Reasoning tokens can be present as a separate field; call this out when relevant instead of silently adding it to the displayed total.

When summarizing, include:

- exact date range and timezone assumption when relevant
- total tokens and cost
- message count
- breakdown requested by the user, usually client/model/workspace/session
- caveats from diagnostics, missing scan paths, or sync-required clients

Keep final answers concise. For large reports, show top contributors and totals rather than dumping full JSON.
