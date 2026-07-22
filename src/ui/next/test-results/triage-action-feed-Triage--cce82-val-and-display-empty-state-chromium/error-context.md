# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: triage-action-feed.spec.ts >> Triage Action Feed UI >> should render triage feed properly, show distinct card types, allow approval, and display empty state
- Location: src/e2e/triage-action-feed.spec.ts:6:7

# Error details

```
Test timeout of 180000ms exceeded.
```

```
Error: locator.fill: Test timeout of 180000ms exceeded.
Call log:
  - waiting for getByPlaceholder('Email or Username')

```

# Page snapshot

```yaml
- generic [ref=e2]: "{\"error\":\"authentication unavailable\"}"
```

# Test source

```ts
  1   | import { expect, test } from '@playwright/test';
  2   |
  3   | test.describe('Triage Action Feed UI', () => {
  4   |   test.use({ viewport: { width: 375, height: 812 } });
  5   |
  6   |   test('should render triage feed properly, show distinct card types, allow approval, and display empty state', async ({ page }) => {
  7   |     test.setTimeout(180000);
  8   |
  9   |     // 1. Log in
  10  |     await page.goto('/login');
> 11  |     await page.getByPlaceholder('Email or Username').fill('test@example.com');
      |                                                      ^ Error: locator.fill: Test timeout of 180000ms exceeded.
  12  |     await page.getByPlaceholder('Password').fill('password123');
  13  |     await page.getByRole('button', { name: 'Log In' }).click();
  14  |     await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });
  15  |
  16  |     // Ensure we are operating as a logged in user, and fetch the tenant
  17  |     const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'e2e-tenant');
  18  |
  19  |     // 2. Seed some distinct triage data matching the backend's expected payload
  20  |     const seedData = [
  21  |       {
  22  |         source: 'Proactive Context Agent',
  23  |         priority: 'high',
  24  |         context: 'Message: Customer asked about vegan cakes.',
  25  |         action_type: 'Draft Reply',
  26  |         action_payload: 'Yes, we have vegan options.',
  27  |         customer_id: 'cust_test_1'
  28  |       },
  29  |       {
  30  |         source: 'Website Booking Request',
  31  |         priority: 'medium',
  32  |         context: 'Booking: Sarah wants an estimate tomorrow at 2PM.',
  33  |         action_type: 'Accept Booking',
  34  |         action_payload: 'Booked for 2PM tomorrow.',
  35  |         customer_id: 'cust_test_2'
  36  |       },
  37  |       {
  38  |         source: 'Inventory Alert',
  39  |         priority: 'urgent',
  40  |         context: 'Alert: Low stock on flour.',
  41  |         action_type: 'Reorder',
  42  |         action_payload: 'Order 50lbs of flour.',
  43  |         customer_id: 'cust_test_3'
  44  |       }
  45  |     ];
  46  |
  47  |     for (const data of seedData) {
  48  |       await page.request.post(`/api/v1/triage/create?tenant_id=${encodeURIComponent(tenantId)}`, {
  49  |         data
  50  |       });
  51  |     }
  52  |
  53  |     // 3. Navigate to Triage Feed
  54  |     await page.goto('/triage');
  55  |     await expect(page.locator('body')).toContainText(/Work Triage/, { timeout: 15000 });
  56  |
  57  |     const listItems = page.locator('div[data-testid^="triage-card-"]');
  58  |
  59  |     await page.waitForTimeout(2000); // Give it a moment to render loaded items
  60  |
  61  |     // Check if we loaded the items properly
  62  |     let count = await listItems.count();
  63  |
  64  |     // 4. Process all cards one by one (approving or dismissing)
  65  |     while (count > 0) {
  66  |       const firstCard = listItems.nth(0);
  67  |       const testId = await firstCard.getAttribute('data-testid');
  68  |       // Click header to expand
  69  |       await firstCard.locator(`[data-testid="triage-card-header-${testId?.replace("triage-card-", "")}"]`).click();
  70  |       await page.waitForTimeout(500);
  71  |
  72  |       // Triage items in the UI are using dynamic IDs
  73  |       const approveBtn = firstCard.locator(`button[data-testid="triage-approve-${testId?.replace("triage-card-", "")}"]`);
  74  |       const reviewBtn = firstCard.locator(`button[data-testid="triage-review-btn-${testId?.replace("triage-card-", "")}"]`);
  75  |       const dismissBtn = firstCard.locator(`button[data-testid="triage-dismiss-${testId?.replace("triage-card-", "")}"]`);
  76  |
  77  |       // We will wait for the API response after clicking
  78  |       const responsePromise = page.waitForResponse(response =>
  79  |         response.url().includes('/api/v1/triage/action') && response.status() === 200
  80  |       ).catch(() => console.log('Response not found or timed out in E2E'));
  81  |
  82  |       try {
  83  |         await approveBtn.waitFor({ state: 'visible', timeout: 2000 });
  84  |         await approveBtn.click();
  85  |       } catch (e) {
  86  |         try {
  87  |           await reviewBtn.waitFor({ state: 'visible', timeout: 2000 });
  88  |           await reviewBtn.click();
  89  |           const saveBtn = firstCard.locator(`button[data-testid="triage-save-btn-${testId?.replace("triage-card-", "")}"]`);
  90  |           await saveBtn.waitFor({ state: 'visible', timeout: 2000 });
  91  |           await saveBtn.click();
  92  |         } catch (e1) {
  93  |           try {
  94  |             await dismissBtn.waitFor({ state: 'visible', timeout: 2000 });
  95  |             await dismissBtn.click();
  96  |           } catch (e2) {
  97  |             console.log(`No approve, review, or dismiss button visible for ${testId?.replace("triage-card-", "")}!`);
  98  |             break;
  99  |           }
  100 |         }
  101 |       }
  102 |
  103 |       await responsePromise;
  104 |
  105 |       // 5. Verify the card disappears (Optimistic UI + backend update)
  106 |       if (testId) {
  107 |           await expect(page.locator(`div[data-testid="${testId}"]`)).not.toBeVisible({ timeout: 5000 });
  108 |       }
  109 |
  110 |       // Re-count remaining cards
  111 |       count = await listItems.count();
```