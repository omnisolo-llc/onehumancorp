# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: visual-workflow.spec.ts >> Visual Workflow Builder E2E >> should allow creating and running a visual workflow
- Location: src/e2e/visual-workflow.spec.ts:4:7

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: getByTestId('visual-workflow-builder')
Expected: visible
Timeout: 5000ms
Error: element(s) not found

Call log:
  - Expect "toBeVisible" with timeout 5000ms
  - waiting for getByTestId('visual-workflow-builder')

```

```yaml
- banner:
  - link "← Back to Dashboard":
    - /url: /dashboard
  - heading "AI Departments" [level=1]
  - heading "Expert Center" [level=2]
  - paragraph: Hire experts, summon expert teams, attach skills and connectors, schedule recurring work, and inspect generated results from one workspace.
  - text: Pro Mode
  - button "Toggle Pro Mode"
  - text: 6 Experts 5 Skills 5 Connectors
  - navigation "Agent feature sections":
    - button "Browse experts" [pressed]
    - button "Expert Teams"
    - button "Skills"
    - button "Connectors"
    - button "Automations"
    - button "Memory"
    - button "Templates"
    - button "Results"
    - button "Activity Feed"
    - button "Needs Approval"
    - button "My Team"
  - text: "Operational team: The Manager The Ambassador The Promoter"
- main:
  - heading "Featured Scenarios" [level=3]
  - text: Marketing Product Swarm
  - heading "Content Creation" [level=4]
  - paragraph: Specialists in posts, copy matching, and social outreach.
  - text: "Includes: Growth Strategist Customer Ambassador Finance Revenue Swarm"
  - heading "Investment Analysis" [level=4]
  - paragraph: Financial margins, Scenario audits, and growth margins.
  - text: "Includes: Finance Controller Revenue Strategist Legal Corporate Counsel"
  - heading "Legal & Compliance" [level=4]
  - paragraph: Legal review, safety checks, and regulatory policy gates.
  - text: "Includes: Operations Manager Policy Checker Ops Operations Swarm"
  - heading "Operations & Supply" [level=4]
  - paragraph: Order dispatch, stock recovery, and team workspace handoffs.
  - text: "Includes: Operations Manager Customer Ambassador"
  - heading "Browse experts" [level=2]
  - paragraph: Search by job, pick a single expert or a coordinated team, then summon it into the task composer.
  - textbox "Search experts":
    - /placeholder: Search experts, roles, skills, connectors...
  - text: 🔍
  - heading "Most used" [level=3]
  - button "Launch Team 812 runs"
  - button "Revenue Rescue Room 574 runs"
  - button "Growth Strategist 428 runs"
  - article:
    - text: GS
    - heading "Growth Strategist" [level=3]
    - text: Expert
    - paragraph: Growth
    - paragraph: Finds high leverage revenue opportunities and turns them into concrete operating tasks.
    - text: positioning offers channel strategy launch planning Use cases
    - list:
      - listitem: • Create a weekend flash sale plan
      - listitem: • Find three low-cost growth loops
      - listitem: • Turn slow sales into a recovery sprint
    - button "Details"
    - button "Summon"
  - article:
    - text: CA
    - heading "Customer Ambassador" [level=3]
    - text: Expert
    - paragraph: Support
    - paragraph: Drafts replies, reads customer context, and escalates sensitive conversations for approval.
    - text: tone matching retention review recovery support triage Use cases
    - list:
      - listitem: • Reply to unhappy customers
      - listitem: • Draft review recovery messages
      - listitem: • Summarize today inbox risk
    - button "Details"
    - button "Summon"
  - article:
    - text: FC
    - heading "Finance Controller" [level=3]
    - text: Expert
    - paragraph: Finance
    - paragraph: Checks margins, cash flow, pricing, and spend before agents commit to expensive actions.
    - text: margin checks cash flow forecasting cost controls Use cases
    - list:
      - listitem: • Audit promotion margin
      - listitem: • Estimate cash impact
      - listitem: • Flag overspend risk
    - button "Details"
    - button "Summon"
  - article:
    - text: OM
    - heading "Operations Manager" [level=3]
    - text: Expert
    - paragraph: Operations
    - paragraph: Coordinates inventory, fulfillment, scheduling, and handoffs between business departments.
    - text: inventory staffing process design handoffs Use cases
    - list:
      - listitem: • Plan order surge staffing
      - listitem: • Create fulfillment checklist
      - listitem: • Find bottlenecks
    - button "Details"
    - button "Summon"
  - complementary:
    - heading "Task Composer" [level=2]
    - paragraph: Growth Strategist is ready
    - text: MiniMax-M3
    - textbox "Task prompt":
      - /placeholder: What can I help you with today? Reference files with @, summon tools with /
      - text: Create a practical operating plan and assign next actions.
    - combobox:
      - option "Ask" [selected]
      - option "Craft"
      - option "Plan"
    - text: Model
    - combobox "Model":
      - option "MiniMax-M3" [selected]
      - option "Auto"
      - option "OpenAI GPT-4.1"
      - option "Claude Sonnet"
      - option "Local Ollama"
    - text: ⚙️ Skills (2) 🔒 Default Safe
    - button "📎"
    - button "🎙️"
    - button "✨"
    - text: Custom provider
    - textbox "Custom provider":
      - /placeholder: OpenAI-compatible URL
    - text: Work directory
    - textbox "Work directory":
      - /placeholder: /workspace/current-task
    - text: Workspace Scoping
    - combobox "Workspace Scoping":
      - option "Current business" [selected]
      - option "Marketing sprint"
      - option "Finance review"
      - option "Customer support"
    - text: Context references (@ tags)
    - textbox "Context references":
      - /placeholder: "@orders @inventory @customer-notes"
    - text: Attachments
    - textbox "Attachments":
      - /placeholder: Drop files, screenshots, PDFs, CSVs
    - button "Screenshot"
    - text: Output format
    - combobox "Output format":
      - option "Brief" [selected]
      - option "Table"
      - option "Document"
      - option "Spreadsheet"
    - text: Task constraints
    - textbox "Task constraints":
      - /placeholder: budget, tone
    - text: Local Ollama Vision Tool use Long context Parallel tasks Active Skills
    - paragraph: Web Research, Campaign Builder
    - text: Active Connectors
    - paragraph: Tencent Docs, Stripe
    - paragraph: "⚠️ Cost warning: Craft and Plan modes can invoke automatic tools and consume more agent budget."
    - button "Start task"
    - heading "Results" [level=2]
    - button "Artifacts" [pressed]
    - button "All files"
    - button "Diffs"
    - button "Preview"
    - text: Artifacts
    - paragraph: Growth Strategist output will appear here after a task starts.
    - button "Share result"
    - button "Download file"
    - button "Copy to workspace"
    - button "Archive task"
    - button "Unarchive"
    - heading "Extensions" [level=2]
    - button "Remote control"
    - button "Data management"
    - button "Workflows"
    - button "Templates"
- button "Help":
  - img
- button "Open help chat": ✨ Ask anything
- button "Voice Assistant":
  - img
- alert
```

# Test source

```ts
  1  | import { test, expect } from '@playwright/test';
  2  |
  3  | test.describe('Visual Workflow Builder E2E', () => {
  4  |   test('should allow creating and running a visual workflow', async ({ page }) => {
  5  |     // Navigate to the agents page
  6  |     await page.goto('/agents');
  7  |
  8  |     // Wait for the workflow builder to be visible
  9  |     const builder = page.getByTestId('visual-workflow-builder');
> 10 |     await expect(builder).toBeVisible();
     |                           ^ Error: expect(locator).toBeVisible() failed
  11 |
  12 |     // Fill in workflow name
  13 |     await page.locator('#visual-workflow-name').fill('My Visual Test Workflow');
  14 |
  15 |     // Add a couple of blocks from the palette
  16 |     await page.getByTestId('palette-block-trigger_message').click();
  17 |     await page.getByTestId('palette-block-action_draft').click();
  18 |
  19 |     // Ensure they appeared on the canvas
  20 |     await expect(page.getByTestId('canvas-block-0')).toBeVisible();
  21 |     await expect(page.getByTestId('canvas-block-1')).toBeVisible();
  22 |
  23 |     // Mock the API response to avoid actual execution if we don't have the backend
  24 |     await page.route('/api/workflow/run', async route => {
  25 |       await route.fulfill({
  26 |         status: 200,
  27 |         contentType: 'application/json',
  28 |         body: JSON.stringify({ success: true, result: 'E2E Visual Workflow Success' }),
  29 |       });
  30 |     });
  31 |
  32 |     // Save and run
  33 |     await page.locator('#btn-create-run-workflow').click();
  34 |
  35 |     // Verify it got added to the list (the API mock should return success)
  36 |     await expect(page.getByText('Visual Workflow Result: E2E Visual Workflow Success')).toBeVisible({ timeout: 10000 });
  37 |   });
  38 | });
  39 |
```