<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# CUJ: Omni-Context Sub-agent Routing

1. An orchestrating agent calls `DelegateMission`.
2. The `SIPDB` checks for `AGENTS.md` or `CLAUDE.md` in the current context root.
3. If found, it injects the content into the task payload before saving to the DB.
4. The assigned sub-agent receives the task with full context.

</div>
