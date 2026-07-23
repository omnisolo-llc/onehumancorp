1. **Database Schema Update:**
   - Create a new Goose migration file in `src/server/db/migrations/`.
   - The migration should extend the `proposals` table to include `project_scope` (TEXT) and `milestones` (JSONB) or create a new `proposal_milestones` table. Given the prompt requests JSONB or separate table, JSONB on `proposals` is often simpler for structured milestones, but let's see how `proposals` works currently. Let's add columns to `proposals`: `project_scope TEXT` and `milestones JSONB`.
   - Update `src/server/api/proposals.rs` to reflect these new fields in the `Proposal` struct.

2. **API Endpoint:**
   - In `src/server/api/proposals.rs`, add a new POST endpoint `/api/v1/client-inquiries` (or update an existing intake endpoint).
   - This endpoint should accept client inquiry data (e.g., name, email, project description).
   - Route this inquiry to the `SalesAgent` for processing. We'll likely need to invoke an orchestrator or worker or directly call the `SalesAgent` logic to generate the proposal.
   - The `SalesAgent` should generate the proposal text, scope of work, and pricing by referencing similar past projects. We will simulate this or use the LLM if available in the context. Then save the proposal in the database.

3. **Agent Integration (`src/server/orchestration/departments/sales_agent.rs`):**
   - Update `SalesAgent` to handle a `ClientInquiry` event.
   - Add a method to draft a proposal based on the inquiry. It should output a `project_scope` and a list of `milestones`.

4. **UI Updates (Tauri/Next):**
   - Create a new mobile-first UI component in `src/ui/tauri` or `src/ui/next` to allow owners to review, edit, and approve the AI-drafted proposal. This might be part of the "Agent Feed" or a dedicated proposals page.
   - Implement a client-facing, read-only proposal view with Stripe integration for deposit collection.

5. **Testing:**
   - Add unit tests for the new endpoint and Sales Agent logic.
   - Add a Playwright E2E test `src/e2e/playwright/nora-proposal-intake.spec.ts` that simulates a client inquiry, the owner reviewing the drafted proposal in the feed, editing it, and the client viewing and paying the deposit.
