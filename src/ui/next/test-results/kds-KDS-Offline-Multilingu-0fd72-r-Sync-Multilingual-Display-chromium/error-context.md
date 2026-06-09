# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: kds.spec.ts >> KDS Offline & Multilingual >> KDS Order Sync & Multilingual Display
- Location: src/e2e/kds.spec.ts:23:7

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: locator('text=#1 - Ahmed')
Expected: visible
Timeout: 5000ms
Error: element(s) not found

Call log:
  - Expect "toBeVisible" with timeout 5000ms
  - waiting for locator('text=#1 - Ahmed')

```

```yaml
- img
- heading "This page couldn’t load" [level=1]
- paragraph: Reload to try again, or go back.
- button "Reload"
- button "Back"
```

# Test source

```ts
  1  | import { test, expect } from '@playwright/test';
  2  |
  3  | test.describe('KDS Offline & Multilingual', () => {
  4  |   test.describe.configure({ mode: 'serial' });
  5  |
  6  |   test.beforeEach(async ({ page, context, request }) => {
  7  |     // Clear cookies and state
  8  |     await context.clearCookies();
  9  |     await page.goto('/pos/kds');
  10 |     await page.evaluate(() => localStorage.clear());
  11 |
  12 |     // Reset backend state before each test
  13 |     await request.delete('/api/pos/orders');
  14 |     await request.delete('/api/pos/inventory');
  15 |
  16 |     // Reload to ensure fresh state with clean local storage
  17 |     await page.reload();
  18 |
  19 |     // Wait for mock data to load after reload
  20 |     await expect(page.locator('text=Active Orders')).toBeVisible({ timeout: 10000 });
  21 |   });
  22 |
  23 |   test('KDS Order Sync & Multilingual Display', async ({ page }) => {
  24 |
> 25 |     await expect(page.locator('text=#1 - Ahmed')).toBeVisible();
     |                                                   ^ Error: expect(locator).toBeVisible() failed
  26 |     await expect(page.getByText('Chicken Over Rice', { exact: true })).toBeVisible();
  27 |
  28 |     // Toggle language
  29 |     await page.getByTestId('lang-toggle').click();
  30 |
  31 |     // Check Arabic translations
  32 |     await expect(page.locator('text=الطلبات النشطة')).toBeVisible();
  33 |     await expect(page.locator('text=دجاج فوق الرز')).toBeVisible();
  34 |
  35 |     // Check RTL
  36 |     const dir = await page.locator('div[dir="rtl"]').count();
  37 |     expect(dir).toBeGreaterThan(0);
  38 |   });
  39 |
  40 |   test('KDS Offline Actions & Background Sync', async ({ page, context }) => {
  41 |     await expect(page.locator('text=#1 - Ahmed')).toBeVisible();
  42 |     // Verify initial state is "Received" before attempting to click "Prepare"
  43 |     await expect(page.getByTestId('btn-prepare-1')).toBeVisible({ timeout: 5000 });
  44 |
  45 |     // Set network to offline
  46 |     await context.setOffline(true);
  47 |     // Simulate offline event directly in browser to trigger state updates reliably
  48 |     await page.evaluate(() => window.dispatchEvent(new Event('offline')));
  49 |
  50 |     // Wait for UI to reflect offline state
  51 |     await expect(page.locator('text=Offline Mode')).toBeVisible();
  52 |
  53 |     // Perform optimistic action 1: Update order status
  54 |     await page.getByTestId('btn-prepare-1').click();
  55 |     await expect(page.getByTestId('btn-ready-1')).toBeVisible();
  56 |
  57 |     // Perform optimistic action 2: Toggle sold out
  58 |     await page.getByTestId('toggle-soldout-inv_1').click();
  59 |     await expect(page.getByTestId('toggle-soldout-inv_1')).toHaveText('Sold Out');
  60 |
  61 |     // Verify localStorage queued events
  62 |     const events = await page.evaluate(() => JSON.parse(localStorage.getItem('ohc_kds_events') || '[]'));
  63 |     expect(events.length).toBe(2);
  64 |     expect(events[0].type).toBe('UPDATE_ORDER_STATUS');
  65 |     expect(events[1].type).toBe('TOGGLE_SOLD_OUT');
  66 |
  67 |     // Restore network
  68 |     await context.setOffline(false);
  69 |     await page.evaluate(() => window.dispatchEvent(new Event('online')));
  70 |
  71 |     // Expect offline badge to disappear
  72 |     await expect(page.locator('text=Offline Mode')).toBeHidden();
  73 |
  74 |     // Wait for background sync to trigger (interval is 5s) and clear events
  75 |     await expect(async () => {
  76 |       const remainingEvents = await page.evaluate(() => JSON.parse(localStorage.getItem('ohc_kds_events') || '[]'));
  77 |       expect(remainingEvents.length).toBe(0);
  78 |     }).toPass({ timeout: 10000 });
  79 |   });
  80 |
  81 | });
  82 |
```