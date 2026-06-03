# Cloud Bridge Scope

## Overview
The Cloud Bridge is a viral growth loop designed to seamlessly convert users from Standalone Mode (local-first) to Cloud-Native team usage. This document outlines the technical design, data model, APIs, and UI integration for the "Collaborator Invite" mechanism.

## Data Model (PostgreSQL)
We will leverage the existing \`team_invites\` table implemented in \`src/server/services/growth/invites.rs\`.
- \`id\`: Primary Key (String)
- \`team_id\`: The multi-tenant context ID
- \`inviter_id\`: The Standalone user creating the bridge
- \`invitee_id\`: The target collaborator
- \`status\`: PENDING, ACCEPTED
- \`created_at\` / \`updated_at\`

## API Routes (Backend)
The backend API is already structured in \`src/server/api/growth.rs\`:
- \`POST /api/v1/growth/team-invites\`: Creates an invite and logs a Hub event.
- \`GET /api/v1/growth/team-invites\`: Retrieves pending/active invites.
- \`POST /api/v1/growth/team-invites/accept\`: Marks invite as ACCEPTED.

## Next.js UI Touchpoints
1. **Team Dashboard (\`src/ui/next/src/app/team/page.tsx\`)**:
   - The UI already contains the "Cloud Bridge Invite" modal layout using OHC glassmorphism.
   - **Gap to fix**: The UI generates a static URL based on \`localStorage.getItem('tenant')\` instead of hitting the actual backend to create a tracked \`team_invite\`. We will connect the "Invite to Cloud Team" button to the \`POST /api/v1/growth/team-invites\` route so the backend correctly registers the invite.
2. **Invite Landing Page (\`src/ui/next/src/app/invite/[id]/page.tsx\`)**:
   - A new page that resolves the invite link, showing an acceptance screen.
   - When clicked, it calls \`POST /api/v1/growth/team-invites/accept\` with the \`invite_id\`.

## Verification
- We will add an E2E test (\`src/e2e/viral_cloud_bridge.spec.ts\`) verifying that clicking "Invite to Cloud Team" creates the actual database record and that the generated link leads to an acceptance flow.
