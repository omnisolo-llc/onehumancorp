# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: branding-loop.spec.ts >> Branding Growth Loop >> Powered by OHC footer is present and links correctly
- Location: src/e2e/branding-loop.spec.ts:4:9

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: locator('.powered-by-footer a').first()
Expected: visible
Timeout: 5000ms
Error: element(s) not found

Call log:
  - Expect "toBeVisible" with timeout 5000ms
  - waiting for locator('.powered-by-footer a').first()

```

```yaml
- heading "Welcome to OHC Smart Builder" [level=1]
- paragraph: Review and add any extra details to help our AI generate the perfect store.
- text: Your Business Details
- textbox "e.g. I run a mobile dog grooming service in Portland"
- button "Build My Storefront" [disabled]
- button "Help":
  - img
- button "Open help chat": ✨Ask anything
```

# Test source

```ts
  1  | import { test, expect } from '@playwright/test';
  2  |
  3  | test.describe('Branding Growth Loop', () => {
  4  |     test('Powered by OHC footer is present and links correctly', async ({ page }) => {
  5  |         await page.goto('/storefront-builder');
  6  |         await page.evaluate(() => localStorage.setItem('ohc_builder_status', 'draft'));
  7  |         await page.reload();
  8  |
  9  |         const footerLink = page.locator('.powered-by-footer a').first();
> 10 |         await expect(footerLink).toBeVisible();
     |                                  ^ Error: expect(locator).toBeVisible() failed
  11 |         await expect(footerLink).toContainText('Powered by');
  12 |         await expect(footerLink).toContainText('OHC');
  13 |     });
  14 |
  15 |     test('Website Builder also shows Powered by OHC footer', async ({ page }) => {
  16 |         await page.goto('/website-builder');
  17 |         await page.evaluate(() => {
  18 |             const state = JSON.parse(localStorage.getItem('website-builder-storage') || '{"state":{}}');
  19 |             state.state.status = 'draft';
  20 |             localStorage.setItem('website-builder-storage', JSON.stringify(state));
  21 |         });
  22 |         await page.reload();
  23 |
  24 |         const footerLink = page.locator('.powered-by-footer a').first();
  25 |         await expect(footerLink).toBeVisible({ timeout: 15000 });
  26 |     });
  27 | });
  28 |
```