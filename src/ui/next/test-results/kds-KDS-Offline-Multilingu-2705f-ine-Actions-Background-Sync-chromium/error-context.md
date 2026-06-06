# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: kds.spec.ts >> KDS Offline & Multilingual >> KDS Offline Actions & Background Sync
- Location: src/e2e/kds.spec.ts:25:7

# Error details

```
Error: expect(received).toBe(expected) // Object.is equality

Expected: 0
Received: 2

Call Log:
- Timeout 10000ms exceeded while waiting on the predicate
```

# Page snapshot

```yaml
- generic [ref=e1]:
  - generic [ref=e3]:
    - generic [ref=e4]:
      - heading "Kitchen Display System" [level=1] [ref=e6]
      - button "عربي" [ref=e7] [cursor=pointer]
    - generic [ref=e8]:
      - heading "Active Orders" [level=2] [ref=e9]
      - generic [ref=e10]:
        - generic [ref=e11]:
          - generic [ref=e12]:
            - heading "#1 - Ahmed" [level=3] [ref=e13]
            - generic [ref=e14]: Preparing
          - list [ref=e15]:
            - listitem [ref=e16]: • 2x Chicken Over Rice
          - button "Ready" [ref=e18] [cursor=pointer]
        - generic [ref=e19]:
          - generic [ref=e20]:
            - heading "#2 - Sarah" [level=3] [ref=e21]
            - generic [ref=e22]: Preparing
          - list [ref=e23]:
            - listitem [ref=e24]: • 1x Lamb Combo
            - listitem [ref=e25]: • 1x Soda
          - button "Ready" [ref=e27] [cursor=pointer]
      - heading "Menu Items" [level=2] [ref=e28]
      - generic [ref=e29]:
        - generic [ref=e30]:
          - generic [ref=e31]: Chicken Over Rice
          - button "Sold Out" [active] [ref=e32] [cursor=pointer]
        - generic [ref=e33]:
          - generic [ref=e34]: Lamb Combo
          - button "Available" [ref=e35] [cursor=pointer]
  - button "Help" [ref=e38] [cursor=pointer]:
    - img [ref=e39]
  - button "Open help chat" [ref=e42] [cursor=pointer]:
    - generic [ref=e43]: ✨
    - generic: Ask anything
  - alert [ref=e44]
```

# Test source

```ts
  1  | import { test, expect } from '@playwright/test';
  2  |
  3  | test.describe('KDS Offline & Multilingual', () => {
  4  |
  5  |   test('KDS Order Sync & Multilingual Display', async ({ page }) => {
  6  |     await page.goto('/pos/kds');
  7  |
  8  |     // Wait for mock data to load
  9  |     await expect(page.locator('text=Active Orders')).toBeVisible();
  10 |     await expect(page.locator('text=#1 - Ahmed')).toBeVisible();
  11 |     await expect(page.getByText('Chicken Over Rice', { exact: true })).toBeVisible();
  12 |
  13 |     // Toggle language
  14 |     await page.getByTestId('lang-toggle').click();
  15 |
  16 |     // Check Arabic translations
  17 |     await expect(page.locator('text=الطلبات النشطة')).toBeVisible();
  18 |     await expect(page.locator('text=دجاج فوق الرز')).toBeVisible();
  19 |
  20 |     // Check RTL
  21 |     const dir = await page.locator('div[dir="rtl"]').count();
  22 |     expect(dir).toBeGreaterThan(0);
  23 |   });
  24 |
  25 |   test('KDS Offline Actions & Background Sync', async ({ page, context }) => {
  26 |     await page.goto('/pos/kds');
  27 |     await expect(page.locator('text=#1 - Ahmed')).toBeVisible();
  28 |
  29 |     // Set network to offline
  30 |     await context.setOffline(true);
  31 |     // Simulate offline event directly in browser to trigger state updates reliably
  32 |     await page.evaluate(() => window.dispatchEvent(new Event('offline')));
  33 |
  34 |     // Wait for UI to reflect offline state
  35 |     await expect(page.locator('text=Offline Mode')).toBeVisible();
  36 |
  37 |     // Perform optimistic action 1: Update order status
  38 |     await page.getByTestId('btn-prepare-1').click();
  39 |     await expect(page.getByTestId('btn-ready-1')).toBeVisible();
  40 |
  41 |     // Perform optimistic action 2: Toggle sold out
  42 |     await page.getByTestId('toggle-soldout-inv_1').click();
  43 |     await expect(page.getByTestId('toggle-soldout-inv_1')).toHaveText('Sold Out');
  44 |
  45 |     // Verify localStorage queued events
  46 |     const events = await page.evaluate(() => JSON.parse(localStorage.getItem('ohc_kds_events') || '[]'));
  47 |     expect(events.length).toBe(2);
  48 |     expect(events[0].type).toBe('UPDATE_ORDER_STATUS');
  49 |     expect(events[1].type).toBe('TOGGLE_SOLD_OUT');
  50 |
  51 |     // Restore network
  52 |     await context.setOffline(false);
  53 |     await page.evaluate(() => window.dispatchEvent(new Event('online')));
  54 |
  55 |     // Expect offline badge to disappear
  56 |     await expect(page.locator('text=Offline Mode')).toBeHidden();
  57 |
  58 |     // Wait for background sync to trigger (interval is 5s) and clear events
  59 |     await expect(async () => {
  60 |       const remainingEvents = await page.evaluate(() => JSON.parse(localStorage.getItem('ohc_kds_events') || '[]'));
  61 |       expect(remainingEvents.length).toBe(0);
> 62 |     }).toPass({ timeout: 10000 });
     |        ^ Error: expect(received).toBe(expected) // Object.is equality
  63 |   });
  64 |
  65 | });
  66 |
```