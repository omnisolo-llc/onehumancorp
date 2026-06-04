# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: proactive-ambassador.spec.ts >> Proactive Ambassador CUJ >> displays abandoned cart recovery action card and allows approval
- Location: src/e2e/proactive-ambassador.spec.ts:4:7

# Error details

```
Test timeout of 30000ms exceeded.
```

```
Error: locator.click: Test timeout of 30000ms exceeded.
Call log:
  - waiting for getByRole('button', { name: 'Login' })

```

# Page snapshot

```yaml
- generic [ref=e1]:
  - generic [ref=e2]:
    - heading "Login" [level=1] [ref=e3]
    - textbox "Email or Username" [ref=e4]: maya@ohc.test
    - textbox "Password" [active] [ref=e5]: password123
    - button "Log In" [ref=e6] [cursor=pointer]
    - button "Start Business Setup" [ref=e7] [cursor=pointer]
  - button "Help" [ref=e10] [cursor=pointer]:
    - img [ref=e11]
  - button "Open help chat" [ref=e14] [cursor=pointer]:
    - generic [ref=e15]: ✨
    - generic: Ask anything
  - alert [ref=e16]
```

# Test source

```ts
  1  | import { test, expect } from '@playwright/test';
  2  |
  3  | test.describe('Proactive Ambassador CUJ', () => {
  4  |   test('displays abandoned cart recovery action card and allows approval', async ({ page }) => {
  5  |     // Navigate to the login page and authenticate to access the dashboard
  6  |     await page.goto('/login');
  7  |     await page.getByPlaceholder('Email or Username').fill('maya@ohc.test');
  8  |     await page.getByPlaceholder('Password').fill('password123');
> 9  |     await page.getByRole('button', { name: 'Login' }).click();
     |                                                       ^ Error: locator.click: Test timeout of 30000ms exceeded.
  10 |
  11 |     // Verify successful login
  12 |     await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();
  13 |
  14 |     // Intercept /api/agents/approvals to return our mocked Ambassador approval
  15 |     await page.route('/api/agents/approvals*', async route => {
  16 |       const json = {
  17 |         pending_approvals: [
  18 |           {
  19 |             id: 'mock_abandoned_cart_1',
  20 |             tenant_id: 'test_tenant',
  21 |             department: 'The Ambassador',
  22 |             description: 'Recover Abandoned Carts',
  23 |             status: 'pending',
  24 |             action_risk: 'low',
  25 |             payload: {
  26 |               abandoned_carts_count: 3,
  27 |               potential_revenue: 120,
  28 |               draft_message: "Hey there! We noticed you left some items in your cart. Here's a 10% discount to complete your order!"
  29 |             }
  30 |           }
  31 |         ]
  32 |       };
  33 |       await route.fulfill({ json });
  34 |     });
  35 |
  36 |     // Mock the approval POST endpoint
  37 |     await page.route('/api/agents/approvals/mock_abandoned_cart_1', async route => {
  38 |         if (route.request().method() === 'POST') {
  39 |             await route.fulfill({ status: 200, json: { success: true } });
  40 |         } else {
  41 |             await route.continue();
  42 |         }
  43 |     });
  44 |
  45 |     // Go to the dashboard
  46 |     await page.goto('/dashboard');
  47 |
  48 |     // Verify the feed renders the "Agent Proposals" section
  49 |     await expect(page.getByRole('heading', { name: 'Agent Proposals' })).toBeVisible();
  50 |
  51 |     // Verify the department badge
  52 |     await expect(page.getByText('✨ The Ambassador').first()).toBeVisible();
  53 |
  54 |     // Verify the context details from the payload
  55 |     await expect(page.getByText('Abandoned Carts:')).toBeVisible();
  56 |     await expect(page.getByText('3', { exact: true }).first()).toBeVisible();
  57 |
  58 |     await expect(page.getByText('Potential Revenue:')).toBeVisible();
  59 |     await expect(page.getByText('$120')).toBeVisible();
  60 |
  61 |     await expect(page.getByText('Draft Message:')).toBeVisible();
  62 |     await expect(page.getByText('"Hey there! We noticed you left some items in your cart. Here\'s a 10% discount to complete your order!"')).toBeVisible();
  63 |
  64 |     // Ensure all 3 buttons are present
  65 |     const approveButton = page.getByRole('button', { name: 'Approve proposal' });
  66 |     const declineButton = page.getByRole('button', { name: 'Decline proposal' });
  67 |     const editButton = page.getByRole('button', { name: 'Edit proposal' });
  68 |
  69 |     await expect(approveButton).toBeVisible();
  70 |     await expect(declineButton).toBeVisible();
  71 |     await expect(editButton).toBeVisible();
  72 |
  73 |     // Click Approve
  74 |     await approveButton.click();
  75 |
  76 |     // Wait for optimistic update and verify the card is removed and we see "All caught up!"
  77 |     await expect(page.getByText('All caught up!')).toBeVisible();
  78 |     await expect(page.getByText('Your agents are currently monitoring the business.')).toBeVisible();
  79 |   });
  80 | });
  81 |
```