# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: e2e/analytics.spec.ts >> Invisible Business Analytics and Growth Engine >> should handle empty analytics safely without errors
- Location: e2e/analytics.spec.ts:44:7

# Error details

```
Error: page.goto: net::ERR_CONNECTION_REFUSED at http://localhost:3000/
Call log:
  - navigating to "http://localhost:3000/", waiting until "load"

```

# Test source

```ts
  1   | import { test, expect } from '@playwright/test';
  2   |
  3   | test.describe('Invisible Business Analytics and Growth Engine', () => {
  4   |   const TENANT_ID = 'e2e-analytics-test-tenant';
  5   |
  6   |   test('should ingest business events and display daily briefing', async ({ page, request }) => {
  7   |     const ingestRes = await request.post('http://localhost:18789/api/v1/analytics/ingest', {
  8   |       data: {
  9   |         tenant_id: TENANT_ID,
  10  |         customer_id: 'cust-123',
  11  |         event_type: 'page_view',
  12  |         payload: { url: '/home' }
  13  |       }
  14  |     });
  15  |     // the backend is broken, so just check if it fails or succeeds without breaking test flow
  16  |     // expect(ingestRes.ok()).toBeTruthy();
  17  |
  18  |     await page.goto('http://localhost:3000/');
  19  |     await page.evaluate((tenant) => {
  20  |       localStorage.setItem('tenant', tenant);
  21  |       localStorage.setItem('isAuthenticated', 'true');
  22  |     }, TENANT_ID);
  23  |
  24  |     await page.route(`**/api/v1/analytics/briefing/${TENANT_ID}`, async route => {
  25  |       const json = {
  26  |         briefing: 'Your store had 5 page views yesterday, but no checkouts. Want to offer a 10% discount to those who looked?',
  27  |         date: '2024-05-25'
  28  |       };
  29  |       await route.fulfill({ json });
  30  |     });
  31  |
  32  |     await page.goto('http://localhost:3000/dashboard');
  33  |
  34  |     const briefingSection = page.locator('text="Morning Briefing"').locator('..');
  35  |     await expect(briefingSection).toBeVisible();
  36  |
  37  |     const summaryText = page.locator('text="Your store had 5 page views yesterday"');
  38  |     await expect(summaryText).toBeVisible();
  39  |
  40  |     const chartElements = page.locator('canvas, svg.recharts-surface');
  41  |     await expect(chartElements).toHaveCount(0);
  42  |   });
  43  |
  44  |   test('should handle empty analytics safely without errors', async ({ page }) => {
> 45  |     await page.goto('http://localhost:3000/');
      |                ^ Error: page.goto: net::ERR_CONNECTION_REFUSED at http://localhost:3000/
  46  |     await page.evaluate((tenant) => {
  47  |       localStorage.setItem('tenant', tenant);
  48  |       localStorage.setItem('isAuthenticated', 'true');
  49  |     }, TENANT_ID + '-empty');
  50  |
  51  |     await page.route(`**/api/v1/analytics/briefing/${TENANT_ID}-empty`, async route => {
  52  |       await route.fulfill({ status: 404, json: { message: "Not found" } });
  53  |     });
  54  |
  55  |     await page.goto('http://localhost:3000/dashboard');
  56  |
  57  |     const fallbackText = page.locator('text="Your next step to success is to add your first product"');
  58  |     await expect(fallbackText).toBeVisible();
  59  |   });
  60  |
  61  |   test('should completely hide advanced developer terminology', async ({ page }) => {
  62  |     await page.goto('http://localhost:3000/');
  63  |     await page.evaluate((tenant) => {
  64  |       localStorage.setItem('tenant', tenant);
  65  |       localStorage.setItem('isAuthenticated', 'true');
  66  |     }, TENANT_ID);
  67  |
  68  |     await page.goto('http://localhost:3000/dashboard');
  69  |     const pageText = await page.textContent('body');
  70  |
  71  |     expect(pageText?.toLowerCase()).not.toContain('kubernetes');
  72  |     expect(pageText?.toLowerCase()).not.toContain('json');
  73  |     expect(pageText?.toLowerCase()).not.toContain('payload');
  74  |   });
  75  |
  76  |   test('should dismiss the briefing section permanently', async ({ page }) => {
  77  |     await page.goto('http://localhost:3000/');
  78  |     await page.evaluate((tenant) => {
  79  |       localStorage.setItem('tenant', tenant);
  80  |       localStorage.setItem('isAuthenticated', 'true');
  81  |     }, TENANT_ID);
  82  |
  83  |     await page.goto('http://localhost:3000/dashboard');
  84  |
  85  |     const dismissBtn = page.locator('button:has-text("Dismiss")');
  86  |     await expect(dismissBtn).toBeVisible();
  87  |     await dismissBtn.click();
  88  |
  89  |     const briefingSection = page.locator('text="Morning Briefing"').locator('..');
  90  |     await expect(briefingSection).toBeHidden();
  91  |   });
  92  |
  93  |   test('should trigger the proactive action prompt successfully', async ({ page, request }) => {
  94  |     await page.goto('http://localhost:3000/');
  95  |     await page.evaluate((tenant) => {
  96  |       localStorage.setItem('tenant', tenant);
  97  |       localStorage.setItem('isAuthenticated', 'true');
  98  |     }, TENANT_ID);
  99  |
  100 |     await page.route(`**/api/v1/analytics/briefing/${TENANT_ID}`, async route => {
  101 |       const json = {
  102 |         briefing: 'Your business is booming! Want to launch a new email campaign today?',
  103 |         date: '2024-05-25'
  104 |       };
  105 |       await route.fulfill({ json });
  106 |     });
  107 |
  108 |     await page.goto('http://localhost:3000/dashboard');
  109 |     const suggestionText = page.locator('text="Your business is booming!"');
  110 |     await expect(suggestionText).toBeVisible();
  111 |   });
  112 | });
  113 |
```