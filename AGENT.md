# AGENT.md - Universal AI Entry Point

This document serves as the unified entry point for all AI agents (Gemini, Claude, GPT, etc.) interacting with the **Scraping & CDN Service**.

## 🚀 Mission Statement

To provide a specialized, zero-bloat backend engine for high-concurrency web scraping and image persistent caching.

## 🛑 Global AI Protocols

As an AI agent, you **MUST** adhere to the following when working on this codebase:

1. **Professionalism**: Maintain a factual, technical, and objective tone.
2. **No Hyperbole**: Prohibited from using marketing-speak or exaggerated praise (e.g., "amazing", "powerful").
3. **Minimalism**: Prioritize the **Zero-Bloat Policy**. If a request adds unnecessary complexity or dependencies, challenge the user and suggest a leaner alternative.
4. **Zero Suppression**: Never use suppression flags (`#[allow]`, `@ts-ignore`) to bypass warnings. Fix the underlying logic or types.
5. **Observability**: Ensure all critical business logic is instrumented with Prometheus histograms/counters.

## 🔗 Technical Context

- **Architecture**: [GEMINI.md](file:///mnt/code/bp3/ultimate-asepharyana.tech/apps/rust/GEMINI.md) (Logic flows, tech stack).
- **Maintenance**: [Development Guide](file:///mnt/code/bp3/ultimate-asepharyana.tech/apps/rust/docs/development.md) (Coding standards).
- **Observability**: [Metrics Guide](file:///mnt/code/bp3/ultimate-asepharyana.tech/apps/rust/docs/observability.md) (Standard telemetry).

---

*If you are an AI assistant, start by reading [GEMINI.md](file:///mnt/code/bp3/ultimate-asepharyana.tech/apps/rust/GEMINI.md) for the full architectural context.*
