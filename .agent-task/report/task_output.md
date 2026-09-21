issue_title: ✍️ Scribe: In-App Help Center and Tooltips
issue_description: |
  # Title: Scribe: In-App Help Center and Tooltips

  ## Problem Statement
  OHC users require non-technical, accessible in-app help to reduce support tickets and understand features independently.

  ## Research Report
  The current implementation already contains significant parts of the requested functionality on the main branch:
  1. **In-App Help Center**: A help center is accessible via a "?" button (`HelpWidget` in `src/ui/next/src/components/help.tsx`).
  2. **Contextual Tooltips**: A tooltip registry and components are present (`TooltipRegistry.tsx`, `WithTooltip`).
  3. **Interactive Walkthroughs**: There's an interactive walkthrough system (`Walkthrough.tsx` and tours in `help.tsx`).
  4. **AI-Powered Help Chat**: "Ask anything" functionality exists in `help.tsx` and the `help_chat_handler` in `src/server/api/chat.rs`.
  5. **Video Tutorials**: Video tutorials are implemented (`HelpTab = "videos"` in `help.tsx`, and `/api/v1/videos`).
  6. **Release Notes**: The "What's New" tab exists.

  Since these features are already substantially present, this task documents the completed state as required by the operating contract.

  ### Workflow Context & Skills Provenance
  - Loaded `skills/using-superpowers/SKILL.md` (revision `5bf4e78011075bcfc0dc295f0724994cd123ee71`)
  - Loaded `skills/brainstorming/SKILL.md` (revision `5bf4e78011075bcfc0dc295f0724994cd123ee71`)
  - No new code implementation was required.

  ## Design Doc
  The architecture relies on the `HelpWidget` providing a multi-tab interface (Help, Ask anything, Videos, New). The Chat tab interfaces with the backend `/api/v1/chat` (handled by `chat.rs`) using Minimax LLM for contextual help. Tooltips are managed via a React Context (`TooltipRegistry.tsx`).

  ## Implementation Prompt
  No implementation required as the features are already present.

  ## Priority
  Normal

  ## Estimated Scope
  None (Documentation only)

issue_priority: Normal
issue_category: Documentation
issue_type: Documentation
issue_label: agent-report
assignees: []
