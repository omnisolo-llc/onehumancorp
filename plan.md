1. **Explore the requested issue.**
   - Review the requirements for the "Premium Research Doc" to be submitted via PR.
   - Review the requirements for the YAML block needed at the end of the final message.

2. **Generate the Premium Research Doc.**
   - Combine the findings from the `docs/research/agent_harness_audit.md`, `docs/research/agent_harness_network_proxy_audit.md`, `docs/research/claude_code_harness_audit.md`, `docs/architecture/claude_code_harness_research.md`, and `docs/research/agent-harness-analysis.md` into a single, cohesive, premium-styled markdown file (with the `backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif;` styling, as per OHC Visual Excellence Mandate).
   - Let's call this new doc `docs/research/AGENT_HARNESS_REPORT.md`.

3. **Generate the Mission File.**
   - The mission requires creating a mission file in `.agent-task/missions/{timestamp}.md` that includes Title, Problem Statement, Research Report, Design Doc, Implementation Prompt, Priority, and Estimated Scope. I've already done this step!

4. **Add and commit files.**
   - Add `docs/research/AGENT_HARNESS_REPORT.md` and `.agent-task/missions/*.md` using `git add -f`.

5. **Complete pre-commit steps.**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

6. **Submit PR.**
   - Use `submit` with the correct branch format `Oracle: [Architecture] description`.

7. **Output the YAML block.**
   - In the final message, output the required YAML block to trigger the automator.
