# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: orders-offline.spec.ts >> Offline-First Order Sync Engine >> optimistic UI updates and offline indicator work
- Location: src/e2e/orders-offline.spec.ts:4:7

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator:  locator('[data-testid="offline-indicator"]')
Expected: visible
Received: hidden
Timeout:  15000ms

Call log:
  - Expect "toBeVisible" with timeout 15000ms
  - waiting for locator('[data-testid="offline-indicator"]')
    33 × locator resolved to <span data-testid="offline-indicator" class="text-[#FF3B30] font-bold text-sm bg-red-100 px-2 py-1 rounded-md">Offline ☁️</span>
       - unexpected value "hidden"

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
    - link "Lead Magnets":
      - /url: /lead-magnet-generator
    - link "Settings":
      - /url: /settings
    - link "AI Usage":
      - /url: /ai-usage-paywall
    - link "What's New":
      - /url: /changelog
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
    - link "Help":
      - /url: /help
- banner:
  - text: "Site: default"
  - heading "Orders Offline" [level=1]
  - paragraph: Use this workspace from the dashboard navigation.
- main
- button "Help":
  - img
- button "Open help chat": ✨ Ask anything
- button "Voice Assistant":
  - img
- text: Working offline. Changes saved.
- alert
```

# Test source

```ts
  1  | import { test, expect } from '@playwright/test';
  2  |
  3  | test.describe('Offline-First Order Sync Engine', () => {
  4  |   test('optimistic UI updates and offline indicator work', async ({ page, context }) => {
  5  |     // 1. Go to the page (online first)
  6  |     await page.goto('/orders-offline');
  7  |
  8  |     // Check it's not offline yet
  9  |     await expect(page.getByTestId('offline-indicator')).not.toBeVisible();
  10 |
  11 |     // 2. Go Offline
  12 |     await context.setOffline(true);
  13 |
  14 |     // Simulate an offline event in the browser
  15 |     await page.evaluate(() => window.dispatchEvent(new Event('offline')));
  16 |
  17 |     // Verify offline indicator
> 18 |     await expect(page.locator('[data-testid="offline-indicator"]')).toBeVisible({ timeout: 15000 });
     |                                                                     ^ Error: expect(locator).toBeVisible() failed
  19 |
  20 |     // The order shouldn't be completed yet, we can check for the complete button
  21 |     // But since fetch failed/mocked, we might not have orders.
  22 |     // Since it's a static UI check for the purpose of the test, let's just click the sold out button.
  23 |
  24 |     const soldOutBtn = page.getByTestId('btn-sold-out');
  25 |     await expect(soldOutBtn).toBeVisible();
  26 |     await soldOutBtn.click();
  27 |
  28 |     // 3. Go back online
  29 |     await context.setOffline(false);
  30 |
  31 |     // Simulate an online event in the browser
  32 |     await page.evaluate(() => window.dispatchEvent(new Event('online')));
  33 |
  34 |     // Verify offline indicator is gone
  35 |     await expect(page.getByTestId('offline-indicator')).not.toBeVisible();
  36 |
  37 |     // Verify sync toast appears
  38 |     // await expect(page.getByTestId('sync-toast')).toBeVisible();
  39 |     // It might disappear too fast in a real test unless mocked, but we can check if it exists or existed
  40 |   });
  41 | });
  42 |
```