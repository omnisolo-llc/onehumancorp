# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: verify-ui.spec.ts >> Verify onboarding UI
- Location: src/e2e/verify-ui.spec.ts:3:5

# Error details

```
Test timeout of 30000ms exceeded.
```

```
Error: locator.click: Test timeout of 30000ms exceeded.
Call log:
  - waiting for locator('button:has-text("Continue")')

```

# Page snapshot

```yaml
- generic [active] [ref=e1]:
  - generic [ref=e4]:
    - generic [ref=e5]: Failed to process business details
    - generic [ref=e6]:
      - img [ref=e8]
      - heading "Tell us about your business" [level=2] [ref=e10]
      - paragraph [ref=e11]: Describe what you do, or paste your Instagram link. Our AI will set up your store automatically.
      - generic [ref=e12]:
        - button "Back" [ref=e13] [cursor=pointer]:
          - img [ref=e14]
          - text: Back
        - heading "Where are you located?" [level=2] [ref=e16]
        - paragraph [ref=e17]: This helps us set up your shipping and tax settings.
        - textbox "e.g. Portland, OR" [ref=e20]: Portland, OR
        - button "Generate My Business" [ref=e22] [cursor=pointer]
  - button "Help" [ref=e25] [cursor=pointer]:
    - img [ref=e26]
  - button "Open help chat" [ref=e29] [cursor=pointer]:
    - generic [ref=e30]: ✨
    - generic: Ask anything
  - alert [ref=e31]
```

# Test source

```ts
  1  | import { test, expect } from '@playwright/test';
  2  |
  3  | test('Verify onboarding UI', async ({ page }) => {
  4  |   await page.goto('http://localhost:3000/onboarding');
  5  |   await page.waitForTimeout(1000);
  6  |   await page.screenshot({ path: 'onboarding-step1.png' });
  7  |
  8  |   // Step 1: Business Name
  9  |   await page.locator('input[placeholder="e.g. Maya\'s Custom Cakes"]').fill('Maya Cakes');
  10 |   await page.locator('button:has-text("Next")').click();
  11 |
  12 |   // Step 2: What do you sell
  13 |   await page.locator('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]').fill('I bake custom vegan cakes in Portland, OR...');
  14 |   await page.locator('button:has-text("Next")').click();
  15 |
  16 |   // Step 3: Location
  17 |   await page.locator('input[placeholder="e.g. Portland, OR"]').fill('Portland, OR');
  18 |
  19 |   await page.locator('button:has-text("Generate My Business")').click();
  20 |   await page.waitForTimeout(1000);
  21 |   await page.screenshot({ path: 'onboarding-step2.png' });
  22 |
> 23 |   await page.locator('button:has-text("Continue")').click();
     |                                                     ^ Error: locator.click: Test timeout of 30000ms exceeded.
  24 |   await page.waitForTimeout(1000);
  25 |   await page.screenshot({ path: 'onboarding-step3.png' });
  26 | });
  27 |
```