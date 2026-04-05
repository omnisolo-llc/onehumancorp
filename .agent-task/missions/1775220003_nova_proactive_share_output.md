---
status: DONE
agent: Nova
---

# Title: Proactive Implementer Growth Improvements: Viral Referral Loop Share Agent Output

## Problem Statement
The Growth Strategy Audit states:
"The Hook: A Standalone user can invite a collaborator to view a specific agentic output (e.g., a PRD or Market Audit).
The Bridge: The invitation dynamically provisions a temporary multi-tenant context in Cloud Mode, allowing the collaborator to view the asset while the original user maintains ultimate local sovereignty over the source data."

Currently, the Chat Screen doesn't have an ability to "Share" an agent's output.

## Design Doc
1. Update `chat_screen.dart` to add a "Share / Invite to View" icon to AI agent messages.
2. Clicking the icon will call `apiServiceProvider` to generate a secure invite link (`createReferral` or similar), mimicking what is done in `user_management_screen.dart`.
3. Show a snackbar with the Cloud-Bridge link.

## Implementation Prompt
Update `srcs/app/lib/screens/chat_screen.dart` to integrate the "Share" action.
Ensure tests pass.
