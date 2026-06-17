# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: ambassador-auto-reply.spec.ts >> Ambassador Auto-Responder CUJ >> Owner sees AI Handled auto-replied message in inbox
- Location: src/e2e/ambassador-auto-reply.spec.ts:4:7

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: getByRole('heading', { name: 'Dashboard' }).first()
Expected: visible
Timeout: 5000ms
Error: element(s) not found

Call log:
  - Expect "toBeVisible" with timeout 5000ms
  - waiting for getByRole('heading', { name: 'Dashboard' }).first()

```

```yaml
- heading "Login" [level=1]
- textbox "Email or Username": test@example.com
- textbox "Password": password123
- button "Log In"
- text: or
- button "Start Business Setup"
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
  3  | test.describe('Ambassador Auto-Responder CUJ', () => {
  4  |   test('Owner sees AI Handled auto-replied message in inbox', async ({ page, request }) => {
  5  |     // 1. Connect Instagram via Integrations
  6  |     // Start from login to satisfy the rules
  7  |     await page.goto('/login');
  8  |     await page.getByPlaceholder('Email or Username').fill('test@example.com');
  9  |     await page.getByPlaceholder('Password').fill('password123');
  10 |     await page.getByRole('button', { name: 'Log In' }).click();
> 11 |     await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();
     |                                                                            ^ Error: expect(locator).toBeVisible() failed
  12 |
  13 |     // Set configuration for auto-reply in backend if possible, or trigger auto-reply
  14 |     const tenantId = 'test-tenant';
  15 |
  16 |     // 2. Trigger the Ambassador's draft reply via a real API call (no mocks)
  17 |     // The CustomerSuccess agent listens for tenant.message.received, which is triggered via the webhook endpoint
  18 |     const webhookPayload = {
  19 |       tenant_id: tenantId,
  20 |       sender_id: 'testuser',
  21 |       message: 'I would like to place an order.',
  22 |       source: 'instagram'
  23 |     };
  24 |
  25 |     const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || process.env.BASE_URL || '';
  26 |     const response = await request.post(`${apiBase}/api/inbox/webhook`, {
  27 |       data: webhookPayload,
  28 |     });
  29 |
  30 |     expect(response.ok()).toBeTruthy();
  31 |
  32 |     // 3. Wait for background task to execute
  33 |     // In our test environment, we wait for a moment so the worker pool handles it
  34 |     await page.waitForTimeout(2000);
  35 |
  36 |     // 4. Check Inbox Page
  37 |     await page.goto('/inbox');
  38 |     await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();
  39 |
  40 |     // Verify "AI Handled" badge shows up
  41 |     const messageLocator = page.locator('.app-list-item', { hasText: 'I would like to place an order.' }).first();
  42 |     await expect(messageLocator).toBeVisible({ timeout: 5000 });
  43 |
  44 |     // Click it
  45 |     await messageLocator.click();
  46 |
  47 |     // Verify detail shows AI Handled
  48 |     await expect(page.locator('.app-panel-body .app-badge', { hasText: 'AI Handled' })).toBeVisible();
  49 |   });
  50 | });
  51 |
```