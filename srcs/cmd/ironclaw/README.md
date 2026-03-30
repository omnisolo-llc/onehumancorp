<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.08); padding: 24px; border-radius: 12px; font-family: 'Outfit', sans-serif; color: #E0E0E0;">

# IronClaw CLI

**IronClaw** is a security and audit-focused agent CLI that integrates with the OneHumanCorp platform. It authenticates against the IronClaw provider, runs static-analysis security scans, and reports findings.

## Architectural Walkthrough

```mermaid
graph TD
    A[User/CI] -->|Executes CLI| B(IronClaw Core)
    B -->|Authentication| C{IronClaw Provider}
    B -->|File Walking| D[Target Path]
    D -->|Static Analysis| E[Scanner Module]
    E -->|Security Findings| F[Output Formatter]
    F -->|JSON / Text| A
```

## Developer Insights
*See `docs/shared_context.md` for ongoing security hardening and technical debt notes.*

</div>
