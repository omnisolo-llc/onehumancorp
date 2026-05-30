# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: documentation_flows.spec.ts >> Documentation Flows >> Help Widget interactions and Videos
- Location: src/e2e/documentation_flows.spec.ts:4:7

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator:  locator('#help-widget-container').first()
Expected: visible
Received: hidden
Timeout:  5000ms

Call log:
  - Expect "toBeVisible" with timeout 5000ms
  - waiting for locator('#help-widget-container').first()
    14 × locator resolved to <div class="relative " id="help-widget-container">…</div>
       - unexpected value "hidden"

```

```yaml
- heading "Help Center" [level=1]
- textbox "Search for help articles..."
- link "Getting Started Learn how to easily set up your store and accept your first payment.":
  - /url: /help/getting-started
  - heading "Getting Started" [level=2]
  - paragraph: Learn how to easily set up your store and accept your first payment.
- link "My Store Add products, track what's in stock, and change how your store looks.":
  - /url: /help/my-store
  - heading "My Store" [level=2]
  - paragraph: Add products, track what's in stock, and change how your store looks.
- link "Getting Paid Set up how you get paid, view deposits, and handle simple taxes.":
  - /url: /help/payments
  - heading "Getting Paid" [level=2]
  - paragraph: Set up how you get paid, view deposits, and handle simple taxes.
- link "Your AI Helpers Learn how to hire AI helpers and give them tasks to do.":
  - /url: /help/ai-agents
  - heading "Your AI Helpers" [level=2]
  - paragraph: Learn how to hire AI helpers and give them tasks to do.
- link "Finding Customers Send emails to customers and grow your business easily.":
  - /url: /help/marketing
  - heading "Finding Customers" [level=2]
  - paragraph: Send emails to customers and grow your business easily.
- link "Account & Billing View your bills, manage your plan, and invite team members.":
  - /url: /help/account-billing
  - heading "Account & Billing" [level=2]
  - paragraph: View your bills, manage your plan, and invite team members.
- button "Help":
  - img
- button "Help" [pressed]
- button "Ask AI"
- button "Videos"
- button "New"
- heading "Help Center" [level=3]
- textbox "Search for help..."
- link "Getting Started":
  - /url: /help/getting-started
  - heading "Getting Started" [level=4]
- paragraph: Learn how to easily set up your store and accept your first payment.
- link "My Store":
  - /url: /help/my-store
  - heading "My Store" [level=4]
- paragraph: Add products, track what's in stock, and change how your store looks.
- link "Getting Paid":
  - /url: /help/payments
  - heading "Getting Paid" [level=4]
- paragraph: Set up how you get paid, view deposits, and handle simple taxes.
- link "Your AI Helpers":
  - /url: /help/ai-agents
  - heading "Your AI Helpers" [level=4]
- paragraph: Learn how to hire AI helpers and give them tasks to do.
- link "Finding Customers":
  - /url: /help/marketing
  - heading "Finding Customers" [level=4]
- paragraph: Send emails to customers and grow your business easily.
- link "Account & Billing":
  - /url: /help/account-billing
  - heading "Account & Billing" [level=4]
- paragraph: View your bills, manage your plan, and invite team members.
- heading "Interactive Tours" [level=3]
- 'button "Tour: Set up your store"'
- 'button "Tour: Accept your first payment"'
- 'button "Tour: Activate your AI Support Agent"'
- 'button "Tour: Virtual Meeting Room & UltraPlan"'
- 'button "Tour: KAIROS AI OS Orchestration"'
- button "Open help chat": ✨ Ask anything
- text: Need help? Click here for guides, videos, and to ask our AI.
- alert
```

# Test source

```ts
  1  | import { test, expect } from '@playwright/test';
  2  |
  3  | test.describe('Documentation Flows', () => {
  4  |   test('Help Widget interactions and Videos', async ({ page }) => {
  5  |     // Wait for the help page to load
  6  |     await page.goto('http://localhost:3000/help');
  7  |
  8  |     // Make sure the title renders
  9  |     await expect(page.locator('h1:has-text("Help Center")')).toBeVisible();
  10 |
  11 |     // Verify the Help floating widget button exists
  12 |     const helpButton = page.locator('button[aria-label="Help"]');
  13 |     await expect(helpButton).toBeVisible();
  14 |
  15 |     // Click the widget to open it
  16 |     await helpButton.click();
  17 |
  18 |     // Ensure the widget container is visible
  19 |     const widgetContainer = page.locator('#help-widget-container').first();
> 20 |     await expect(widgetContainer).toBeVisible();
     |                                   ^ Error: expect(locator).toBeVisible() failed
  21 |
  22 |     // Change tab to Videos using exact text match
  23 |     const videosTab = widgetContainer.locator('button').filter({ hasText: /^Videos$/ });
  24 |     await expect(videosTab).toBeVisible();
  25 |     await videosTab.click();
  26 |
  27 |     // Click on the first video
  28 |     const firstVideo = widgetContainer.locator('div.aspect-\\[9\\/16\\]').first();
  29 |     // Wait for videos to load
  30 |     await expect(firstVideo).toBeVisible();
  31 |     await firstVideo.click();
  32 |
  33 |     // Ensure video player overlay pops up
  34 |     const videoOverlayTitle = page.locator('h3', { hasText: 'How to set up your first store easily' });
  35 |     await expect(videoOverlayTitle).toBeVisible();
  36 |   });
  37 | });
  38 |
```