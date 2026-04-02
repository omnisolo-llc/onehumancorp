<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Customer User Journey (CUJ): Omni-Context Sub-agent Routing

**Persona:** Orchestrator Agent / Human Engineering Lead
**Context:** Creating and delegating a complex Epic to multiple specialized sub-agents.
**Success Metrics:** Sub-agents execute tasks in perfect alignment with project-specific formatting, styling, and coding conventions from the moment they wake up, without explicitly querying for project rules.

## 1. User Journey Overview
The Human Engineering Lead kicks off a massive codebase refactoring project by assigning the Epic to a Principal Orchestrator Agent. The Orchestrator Agent immediately breaks the task down into smaller sub-tasks, spinning up individual sub-agents for UI, Backend, and DevOps tasks. The sub-agents immediately begin writing code correctly aligned with `AGENTS.md` grounding, drastically accelerating project velocity.

## 2. Step-by-Step Experience

### Step 1: The Project Kickoff
*   **Action:** The Human Lead or CEO requests a complex task ("Implement Glassmorphism Dashboard").
*   **Context:** The `docs/public/AGENTS.md` file contains strict stylistic rules: "All UI components must use `backdrop-filter: blur(20px) saturate(200%)` and `Outfit` typography."
*   **System Response:** The Orchestrator Agent analyzes the request and splits it into three tasks.

### Step 2: The Delegation & Injection
*   **Action:** The Orchestrator Agent uses the internal Swarm Intelligence Protocol (SIP) via `DelegateMission` to assign Task 1 to the UI Sub-agent.
*   **Context:** The SIP backend intercepts the delegation payload.
*   **System Response:** The SIP reads the `AGENTS.md` from the project's root and automatically injects its contents into the task instructions under the `[SYSTEM GROUNDING]` prefix. The modified payload is written to `agent_missions`.

### Step 3: Zero-Latency Awakening
*   **Action:** The UI Sub-agent initializes, fetching its newly created mission from the database.
*   **Context:** The agent sees its core task ("Build Dashboard") and the appended `[SYSTEM GROUNDING]` instructions natively within its prompt.
*   **System Response:** The UI Sub-agent begins coding the UI component, immediately applying the correct `blur(20px) saturate(200%)` styling tokens and `Outfit` font family.

### Step 4: Seamless Execution
*   **Action:** The sub-agent completes the task and writes its test plan.
*   **Context:** The sub-agent also implicitly followed the test-driven development mandate outlined in `AGENTS.md`.
*   **System Response:** The Orchestrator Agent reviews the PR, confirming perfect alignment with project standards on the first attempt. No corrective feedback loop ("Hey, you forgot to read AGENTS.md!") is required.

## 3. Alternative Scenarios

### Scenario A: No Grounding File
*   **Action:** The SIP checks the context root for `AGENTS.md` or `CLAUDE.md`. Neither is found.
*   **System Response:** The SIP gracefully falls back to the original task payload without the `[SYSTEM GROUNDING]` suffix. The agent operates normally, relying only on its pre-configured system prompt.

### Scenario B: Both Files Exist
*   **Action:** The context root contains both `AGENTS.md` and `CLAUDE.md`.
*   **System Response:** The SIP prioritizes `AGENTS.md` due to the strict sequential fallback search in the `DelegateMission` pipeline. Only `AGENTS.md` is injected to prevent token bloat.

## 4. Business Value
This journey eliminates the "context discovery" phase that wastes valuable tokens and wall-clock time in multi-agent workflows. It enforces perfect deterministic alignment, drastically reducing the hallucination rate of sub-agents and minimizing the need for Human-in-the-Loop corrective action.

</div>
