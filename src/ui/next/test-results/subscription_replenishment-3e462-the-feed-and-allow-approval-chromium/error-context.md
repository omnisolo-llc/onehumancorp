# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: subscription_replenishment_feed.spec.ts >> Subscription Replenishment Engine Feed E2E >> should display subscription replenishment recommendation in the feed and allow approval
- Location: src/e2e/subscription_replenishment_feed.spec.ts:6:7

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: getByText(/Autopilot Recommendation/i)
Expected: visible
Timeout: 15000ms
Error: element(s) not found

Call log:
  - Expect "toBeVisible" with timeout 15000ms
  - waiting for getByText(/Autopilot Recommendation/i)

```

```yaml
- dialog "Unhandled Runtime Error":
  - navigation:
    - button "previous" [disabled]:
      - img "previous"
    - button "next" [disabled]:
      - img "next"
    - text: 1 of 1 error Next.js (14.2.35) is outdated
    - link "(learn more)":
      - /url: https://nextjs.org/docs/messages/version-staleness
  - button "Close"
  - heading "Unhandled Runtime Error" [level=1]
  - paragraph: "TypeError: Cannot read properties of undefined (reading 'replace')"
  - heading "Source" [level=2]
  - link "src/app/feed/page.tsx (263:354) @ replace":
    - text: src/app/feed/page.tsx (263:354) @ replace
    - img
  - text: "261 | <span className={`text-[11px] font-bold uppercase tracking-wider ${isDisputeResolution ? 'text-[#FF9500] dark:text-[#FF9F0A]' : 'text-[#0066FF] dark:text-[#0071E3]'} flex items-center gap-1.5`}> 262 | <span className={`w-2 h-2 rounded-full ${isDisputeResolution ? 'bg-[#FF9500] dark:bg-[#FF9F0A]' : 'bg-[#0066FF] dark:bg-[#0071E3]'} opacity-80`}></span> > 263 | {isDisputeResolution ? 'DISPUTE RESOLUTION' : isAmbassador ? 'CUSTOMER MESSAGE' : item.proposed_action?.action_type === 'Draft Quote' ? 'SMART ESTIMATE' : item.proposed_action?.action_type === 'Draft Follow-up' ? 'DEPOSIT FOLLOW-UP' : item.proposed_action?.action_type === 'Draft Booking' ? 'NEW BOOKING REQUEST' : item.event_source.replace(/_/g, ' ')} | ^ 264 | </span> 265 | <span className=\"text-[11px] text-gray-400 font-medium\"> 266 | {new Date(item.created_at).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}"
  - heading "Call Stack" [level=2]
  - button "Show collapsed frames"
```

# Test source

```ts
  1  | import { expect, test } from '@playwright/test';
  2  |
  3  | test.describe('Subscription Replenishment Engine Feed E2E', () => {
  4  |   test.use({ viewport: { width: 375, height: 812 } });
  5  |
  6  |   test('should display subscription replenishment recommendation in the feed and allow approval', async ({ page }) => {
  7  |     test.setTimeout(180000);
  8  |
  9  |     // 1. Log in
  10 |     await page.goto('/login');
  11 |     await page.getByPlaceholder('Email or Username').fill('test@example.com');
  12 |     await page.getByPlaceholder('Password').fill('password123');
  13 |     await page.getByRole('button', { name: 'Log In' }).click();
  14 |     await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });
  15 |
  16 |     // Navigate to the unified agent feed
  17 |     await page.goto('/feed');
  18 |
  19 |     await page.route('**/api/agent-feed*', async (route) => {
  20 |       const json = {
  21 |         items: [
  22 |           {
  23 |             id: 'req_replenish_123',
  24 |             tenant_id: 't_1',
  25 |             lifecycle_state: 'PENDING',
  26 |             feature_type: 'subscription_replenishment',
  27 |             proposed_action: {
  28 |               action_type: 'email',
  29 |               context: 'Based on this customer\'s order history and the estimated consumption rate, they are due for a replenishment. Would you like me to generate a personalized checkout link and draft an email suggesting they refill?'
  30 |             },
  31 |             context_payload: {
  32 |               feature_type: 'subscription_replenishment',
  33 |               customer_name: 'Maya Baker'
  34 |             },
  35 |             created_at: new Date().toISOString()
  36 |           }
  37 |         ],
  38 |       };
  39 |       await route.fulfill({ json });
  40 |     });
  41 |
  42 |     // Reload to apply the route interception
  43 |     await page.reload();
  44 |
  45 |     // Verify the subscription replenishment card is visible
  46 |     const replenishCardText = page.getByText(/Autopilot Recommendation/i);
> 47 |     await expect(replenishCardText).toBeVisible({ timeout: 15000 });
     |                                     ^ Error: expect(locator).toBeVisible() failed
  48 |
  49 |     const recommendationText = page.getByText(/due for a replenishment/i);
  50 |     await expect(recommendationText).toBeVisible();
  51 |
  52 |     // Verify buttons are rendered correctly
  53 |     const approveBtn = page.getByTestId('approve-subscription-replenishment');
  54 |     await expect(approveBtn).toBeVisible();
  55 |     await expect(approveBtn).toHaveText('Generate & Send Email');
  56 |
  57 |     // Setup route interception for the approval decision endpoint
  58 |     await page.route('**/api/agent-feed/req_replenish_123/state', async (route) => {
  59 |       await route.fulfill({ status: 200, json: { success: true } });
  60 |     });
  61 |
  62 |     // Click the approve button
  63 |     await approveBtn.click();
  64 |   });
  65 | });
  66 |
```