# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: onboarding.spec.ts >> Onboarding Wizard Flow >> completes full onboarding flow
- Location: src/e2e/onboarding.spec.ts:4:7

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: locator('text="Review Details"')
Expected: visible
Timeout: 5000ms
Error: element(s) not found

Call log:
  - Expect "toBeVisible" with timeout 5000ms
  - waiting for locator('text="Review Details"')

```

```yaml
- text: Failed to process business details
- img
- heading "Tell us about your business" [level=2]
- paragraph: Describe what you do, or paste your Instagram link. Our AI will set up your store automatically.
- button "Back":
  - img
  - text: Back
- heading "Where are you located?" [level=2]
- paragraph: This helps us set up your shipping and tax settings.
- textbox "e.g. Portland, OR": Portland, OR
- button "Generate My Business"
- button "Help":
  - img
- button "Open help chat": ✨ Ask anything
- alert
```

# Test source

```ts
  1  | import { test, expect } from '@playwright/test';
  2  |
  3  | test.describe('Onboarding Wizard Flow', () => {
  4  |   test('completes full onboarding flow', async ({ page }) => {
  5  |     // Navigate to onboarding page
  6  |     await page.goto('http://localhost:3000/onboarding');
  7  |
  8  |     // Step 1: Business Name
  9  |     await expect(page.locator('text="Tell us about your business"')).toBeVisible();
  10 |     await expect(page.locator('text="What\'s the name of your business?"')).toBeVisible();
  11 |     await page.locator('input[placeholder="e.g. Maya\'s Custom Cakes"]').fill('Maya Cakes');
  12 |     await page.locator('button:has-text("Next")').click();
  13 |
  14 |     // Step 2: What do you sell
  15 |     await expect(page.locator('text="What do you sell?"')).toBeVisible();
  16 |     await page.locator('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]').fill('I bake custom vegan cakes in Portland, OR...');
  17 |     await page.locator('button:has-text("Next")').click();
  18 |
  19 |     // Step 3: Location
  20 |     await expect(page.locator('text="Where are you located?"')).toBeVisible();
  21 |     await page.locator('input[placeholder="e.g. Portland, OR"]').fill('Portland, OR');
  22 |
  23 |     // Click Generate
  24 |     await page.locator('button:has-text("Generate My Business")').click();
  25 |
  26 |     // 2. Wait for Review Details Step
> 27 |     await expect(page.locator('text="Review Details"')).toBeVisible({ timeout: 5000 });
     |                                                         ^ Error: expect(locator).toBeVisible() failed
  28 |
  29 |     // Continue to next step
  30 |     await page.locator('button:has-text("Continue")').click();
  31 |
  32 |     // 3. Wait for Style & Team Step
  33 |     await expect(page.locator('text="Style & Team"')).toBeVisible({ timeout: 5000 });
  34 |
  35 |     // Select Template and Launch
  36 |     await page.locator('text="Classic"').click();
  37 |     await page.locator('button:has-text("Launch Store")').click();
  38 |
  39 |     // 4. Loading screen
  40 |     await expect(page.locator('text="Building Your Business..."')).toBeVisible({ timeout: 5000 });
  41 |
  42 |     // 5. Live Screen
  43 |     await expect(page.locator('text="You\'re Live!"')).toBeVisible({ timeout: 10000 });
  44 |     await expect(page.locator('text="Your business has been successfully launched."')).toBeVisible();
  45 |     await expect(page.locator('text="my-business.ohc.store"')).toBeVisible();
  46 |
  47 |     const dashboardLink = page.locator('a:has-text("Go to Dashboard")');
  48 |     await expect(dashboardLink).toBeVisible();
  49 |     await expect(dashboardLink).toHaveAttribute('href', '/dashboard');
  50 |
  51 |     await dashboardLink.click();
  52 |     await page.waitForURL('**/dashboard');
  53 |
  54 |     await expect(page.locator('text="Morning Briefing"')).toBeVisible();
  55 |     await expect(page.locator('a:has-text("Add your first product")')).toBeVisible();
  56 |   });
  57 | });
  58 |
```