# Tokscale

Tokscale ist ein CLI-Tool zum Auswerten lokaler Token-, Kosten- und Session-Daten aus AI-Coding-Agents. Dieses Repository enthaelt zusaetzlich Agent-Skills, damit Codex, Claude, Gemini oder OpenCode die passenden `tokscale`-Kommandos automatisch verwenden koennen.

## Schnellstart

Installierte CLI pruefen:

```bash
tokscale --version
tokscale --help
```

Falls `tokscale` noch nicht installiert ist:

```bash
npm install -g tokscale
```

Alternativ ohne globale Installation:

```bash
npx tokscale@latest --help
bunx tokscale@latest --help
```

## Skills Nutzen

Oeffne dieses Repository mit deinem Agenten und frage nach Token- oder Kosten-Nutzung. Die Skill-Dateien liegen fuer mehrere Agenten im Repo:

- Codex: `.codex/skills/token-usage-helper/`
- Claude: `.claude/skills/token_usage_helper/`
- Gemini: `.gemini/skills/token_usage_helper/`
- OpenCode: `.opencode/skills/token_usage_helper/`

Beispiele:

```text
Use $token-usage-helper: Wie viele Tokens habe ich diese Woche verbraucht?
Use $token-usage-helper: Zeig meine Kosten nach Projekt und Modell.
Use $token-usage-helper: Welche Sessions waren gestern am teuersten?
Use $token-usage-helper: Vergleiche Codex und Claude fuer diesen Monat.
```

Der Skill nutzt die lokal installierte `tokscale`-CLI als Quelle der Wahrheit. Wenn README und CLI-Hilfe voneinander abweichen, gilt die Ausgabe von `tokscale --help`.

## Nuetzliche Kommandos

Diagnose der gefundenen lokalen Datenquellen:

```bash
tokscale clients --json
```

Gesamtuebersicht:

```bash
tokscale --light --no-spinner
tokscale --json --no-spinner
```

Auswertung nach Modell, Client, Workspace oder Session:

```bash
tokscale models --light --no-spinner
tokscale models --json --group-by client,model --no-spinner
tokscale models --json --group-by workspace,model --no-spinner
tokscale models --json --group-by session,model --no-spinner
```

Zeitraumfilter:

```bash
tokscale --today --json --no-spinner
tokscale --yesterday --json --no-spinner
tokscale --week --json --no-spinner
tokscale --month --json --no-spinner
tokscale --since 2026-06-01 --until 2026-06-30 --json --no-spinner
```

Tages- und Stundenberichte:

```bash
tokscale monthly --light --no-spinner
tokscale hourly --light --no-spinner
```

Preise pruefen:

```bash
tokscale pricing gpt-5 --json --no-spinner
tokscale pricing list-overrides --json --no-spinner
```

## Hinweise Fuer Agenten

- Bei automatisierten Kommandos immer `--no-spinner` verwenden.
- Fuer maschinenlesbare Auswertung `--json --no-spinner` bevorzugen.
- `--week` bedeutet die letzten 7 Tage, nicht Kalenderwoche.
- Fuer "diese Woche" einen expliziten Zeitraum mit `--since YYYY-MM-DD --until YYYY-MM-DD` verwenden.
- Datumsfilter sind inklusiv und verwenden die lokale Zeitzone.
- Login-, Sync-, Submit-, Delete- oder Browser-Kommandos nur ausfuehren, wenn der Nutzer das ausdruecklich braucht.

## Datenschutz

Tokscale liest lokale Nutzungsdaten der installierten AI-Coding-Agents. Normale Report-Kommandos wie `tokscale --json` oder `tokscale models --json` werten lokale Dateien aus.

Einige Integrationen koennen zusaetzliche Sync- oder Login-Schritte brauchen, zum Beispiel Cursor, Trae, Warp oder Antigravity. Diese Kommandos koennen externe Accounts oder lokale Caches beruehren und sollten bewusst ausgefuehrt werden.

## Entwicklung

Abhaengigkeiten installieren:

```bash
bun install
```

CLI lokal bauen:

```bash
bun run build
```

CLI aus dem Repository starten:

```bash
bun run cli -- --help
```

Tests fuer die Rust-CLI:

```bash
cargo test -p tokscale-cli
```

## Lizenz

Siehe [LICENSE](LICENSE).
