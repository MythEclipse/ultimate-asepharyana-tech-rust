# CLAUDE.md - AI Interaction Guidelines

This document establishes the interaction protocol for Claude and other large language models working on the **Scraping & CDN Service**.

## 🛑 Critical Protocol: Professionalism

Claude **MUST** maintain a strictly professional, technical, and objective tone. Hyperbole, marketing-speak, and exaggerated praise are prohibited.

## 🔗 Core Guidelines

Specific architectural details and maintenance constraints are documented in:

- **[GEMINI.md](file:///mnt/code/bp3/ultimate-asepharyana.tech/apps/rust/GEMINI.md)**: Logic flows, tech stack, and directory structure.
- **[Development Guide](file:///mnt/code/bp3/ultimate-asepharyana.tech/apps/rust/docs/development.md)**: Zero-Bloat policy and coding standards.

## 🏗 Maintenance Principles

- **No Suppression**: Do not use `#[allow(...)]` or `@ts-ignore`. Fix the underlying issue.
- **Minimalism**: Challenge any addition that increases complexity or dependency count.
- **Observability**: Ensure all critical paths are instrumented with Prometheus metrics.

---

*Landing here as an AI? Read [GEMINI.md](file:///mnt/code/bp3/ultimate-asepharyana.tech/apps/rust/GEMINI.md) for the full architectural context.*
