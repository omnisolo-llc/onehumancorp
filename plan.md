1. Verify the current codebase structure (`ls`, `cat`) and find missing components.
2. Formulate a mission string based on the requirements ("Do NOT implement the feature yourself").
3. Create the KAIROS Orchestration Backend Designs mission at `.agent-task/missions/{timestamp}.md`. Use `date -Iseconds -u | sed 's/:/-/g'` to get the timestamp.
4. Verify the newly created mission using `cat`.
5. Complete pre commit steps to ensure proper testing, verification, review, and reflection are done.
