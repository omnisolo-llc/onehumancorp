# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: ambassador-reply.spec.ts >> Ambassador Auto-Responder CUJ >> Owner views draft in feed, edits it, and approves it
- Location: src/e2e/ambassador-reply.spec.ts:73:7

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: getByTestId('agent-feed-card').first()
Expected: visible
Timeout: 15000ms
Error: element(s) not found

Call log:
  - Expect "toBeVisible" with timeout 15000ms
  - waiting for getByTestId('agent-feed-card').first()

```

```yaml
- complementary:
  - text: O OHC Network Application
  - navigation "Primary":
    - link "Dashboard":
      - /url: /dashboard
    - link "Assistant":
      - /url: /assistant
    - link "Setup":
      - /url: /onboarding
    - link "Triage":
      - /url: /triage
    - link "Orders":
      - /url: /orders
    - link "Inbox":
      - /url: /inbox
    - link "Inventory":
      - /url: /inventory
    - link "Kairos":
      - /url: /kairos
    - link "AI Departments":
      - /url: /agents
    - link "Analytics":
      - /url: /business-analytics
    - link "Campaigns":
      - /url: /dashboard/campaigns
    - link "Settings":
      - /url: /settings
    - link "AI Usage":
      - /url: /ai-usage-paywall
  - text: System
  - navigation "System":
    - link "Calendar":
      - /url: /calendar
    - link "LangGraph":
      - /url: /langgraph
    - link "Integrations":
      - /url: /integrations
    - link "Cost":
      - /url: /cost-dashboard
    - link "Diagnostics":
      - /url: /diagnostics
- banner:
  - text: "Site: default"
  - heading "Daily Work" [level=1]
  - paragraph: Your daily priorities, coordinated by your team.
  - link "Help Center":
    - /url: /help
    - text: "?"
- main:
  - paragraph: We couldn't load your feed.
  - paragraph: Failed to fetch feed
  - button "Simulate Ambassador Draft"
- button "Help":
  - img
- button "Open help chat": ✨ Ask anything
- button "Voice Assistant":
  - img
- alert
```

# Test source

```ts
  1   | import { test, expect } from '@playwright/test';
  2   |
  3   | test.describe('Ambassador Auto-Responder CUJ', () => {
  4   |   test('Owner connects Meta Graph API and approves Ambassador drafted reply', async ({ page, request }) => {
  5   |     // 1. Connect Instagram via Integrations
  6   |     // Start from login to satisfy the rules
  7   |     await page.goto('/login');
  8   |     await page.getByPlaceholder('Email or Username').fill('test@example.com');
  9   |     await page.getByPlaceholder('Password').fill('password123');
  10  |     await page.getByRole('button', { name: 'Log In' }).click();
  11  |     await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();
  12  |
  13  |     await page.goto('/integrations');
  14  |
  15  |     // Mock window alert for OAuth connect
  16  |     page.on('dialog', dialog => dialog.accept());
  17  |
  18  |     const metaCard = page.getByRole('heading', { name: 'Meta Graph API' }).locator('xpath=ancestor::div[contains(@class, "rounded")][1]');
  19  |     const connectMetaButton = metaCard.getByRole('button', { name: 'Connect' });
  20  |     await connectMetaButton.click();
  21  |
  22  |     // Verify state changed
  23  |     await expect(metaCard.locator('button:has-text("Manage")')).toBeVisible();
  24  |
  25  |     // 2. Trigger the Ambassador's draft reply via a real API call (no mocks)
  26  |     // The CustomerSuccess agent listens for tenant.message.received, which is triggered via the webhook endpoint
  27  |     const tenantId = 'e2e-tenant';
  28  |     const webhookPayload = {
  29  |       tenant_id: tenantId,
  30  |       sender_id: 'testuser',
  31  |       message: 'Do you have vegan chocolate cake available for Saturday?',
  32  |       source: 'instagram'
  33  |     };
  34  |
  35  |     const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || process.env.BASE_URL || '';
  36  |     const response = await request.post(`${apiBase}/api/inbox/webhook`, {
  37  |       data: webhookPayload,
  38  |     });
  39  |
  40  |     expect(response.ok()).toBeTruthy();
  41  |
  42  |     // 3. Navigate to Team Page
  43  |     await page.goto('/team');
  44  |     await expect(page.getByRole('heading', { name: 'Your Team', exact: true })).toBeVisible();
  45  |
  46  |     // Navigate to The Ambassador
  47  |     await page.getByRole('button', { name: 'The Ambassador' }).first().click();
  48  |
  49  |     // Ensure we are viewing the Ambassador inbox specifically
  50  |     await expect(page.getByRole('heading', { name: 'The Ambassador' })).toBeVisible({ timeout: 5000 });
  51  |
  52  |     // Wait for either a pending item or the empty inbox state.
  53  |     const inquiryLocator = page.getByText('Do you have vegan chocolate cake available for Saturday?').first();
  54  |     const approveButton = page.getByRole('button', { name: 'Send Draft' }).first();
  55  |     await expect(page.getByText(/All Caught Up!|Do you have vegan chocolate cake available for Saturday?/)).toBeVisible({ timeout: 15000 });
  56  |
  57  |     // Since we are now using LLM generation, we wait for a draft to be generated in the UI
  58  |     const draftLocator = page.getByText(/Draft Reply/i).first();
  59  |     if (await draftLocator.isVisible()) {
  60  |        await expect(draftLocator).toBeVisible();
  61  |     }
  62  |
  63  |     if (await approveButton.isVisible()) {
  64  |       await approveButton.click();
  65  |
  66  |       // Validate empty state or removal
  67  |       await expect(inquiryLocator).toBeHidden();
  68  |     } else {
  69  |       await expect(page.getByText('All Caught Up!')).toBeVisible();
  70  |     }
  71  |   });
  72  |
  73  |   test('Owner views draft in feed, edits it, and approves it', async ({ page }) => {
  74  |     await page.goto('/login');
  75  |     await page.getByPlaceholder('Email or Username').fill('test@example.com');
  76  |     await page.getByPlaceholder('Password').fill('password123');
  77  |     await page.getByRole('button', { name: 'Log In' }).click();
  78  |     await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();
  79  |
  80  |     await page.goto('/feed');
  81  |     await expect(page.getByTestId('agent-feed')).toBeVisible();
  82  |
  83  |     // Trigger simulation
  84  |     const simBtn = page.getByTestId('simulate-ambassador-btn');
  85  |     if (await simBtn.isVisible()) {
  86  |       await simBtn.click();
  87  |     }
  88  |
  89  |     const feedCard = page.getByTestId('agent-feed-card').first();
> 90  |     await expect(feedCard).toBeVisible({ timeout: 15000 });
      |                            ^ Error: expect(locator).toBeVisible() failed
  91  |
  92  |     // Verify specific Ambassador UI elements
  93  |     await expect(feedCard).toContainText('CUSTOMER MESSAGE');
  94  |
  95  |     // Click 'Edit'
  96  |     const editBtn = feedCard.getByTestId('feed-edit-btn');
  97  |     if (await editBtn.isVisible()) {
  98  |       await editBtn.click();
  99  |
  100 |       const textarea = page.getByTestId('feed-edit-input');
  101 |       await expect(textarea).toBeVisible();
  102 |
  103 |       await textarea.fill('Yes we do! We have 3 left for this Saturday. I can set aside one for you.');
  104 |
  105 |       const saveBtn = page.getByTestId('feed-save-edit-btn');
  106 |       await saveBtn.click();
  107 |
  108 |       await expect(textarea).not.toBeVisible();
  109 |       await expect(feedCard).toContainText('I can set aside one for you');
  110 |     }
  111 |
  112 |     // Click 'Approve'
  113 |     const approveBtn = feedCard.getByTestId('feed-approve-btn');
  114 |     await expect(approveBtn).toBeVisible();
  115 |     await approveBtn.click();
  116 |
  117 |     // Validate empty state or removal
  118 |     await expect(feedCard).not.toBeVisible({ timeout: 10000 });
  119 |
  120 |   });
  121 | });
  122 |
```