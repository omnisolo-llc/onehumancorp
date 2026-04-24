1. **Analyze User and Instructions**: Understand the product vision, target personas, competitive landscape, and task requirements as outlined in the prompt and existing documentation.

2. **Conduct Competitive & Market Research**: Use bash commands (e.g., `grep`, `cat`, `find`) to search for and read existing research documents related to competitors (Shopify, Wix, Squarespace, GoDaddy) and market trends within the repository.

3. **Synthesize Findings & Draft the Output**: Consolidate the findings into the requested output format. Create the main report at `.agent-task/report/task_output.md` containing the Deep Competitor Audit, Persona Pain Points, AI Differentiation Manifesto, Market Sizing, and Feature Gap Matrix. Use Mermaid.js charts for visual representation.

4. **Create GitHub Issue Briefs**: Create a corresponding issue brief inside the `docs/research/` directory. For example, `docs/research/[onboarding]_mobile_first_ai_onboarding.md`. Ensure this document adheres to the specified Markdown structure and premium aesthetic (glassmorphism tokens, etc.).

5. **Complete pre commit steps**: Complete pre commit steps to ensure proper testing, verification, review, and reflection are done. Use the `pre_commit_instructions` tool to fetch these steps. In this case, since there are no code changes, testing might be skipped or limited to verifying the markdown files exist and are formatted correctly.

6. **Submit**: Once all steps are complete and the output files are verified, use the `submit` tool to finalize the task. No tests need to be run as this is a research task.

Note: Steps 1-4 have already been executed via bash commands. The plan formalizes these actions and adds the necessary final steps.
