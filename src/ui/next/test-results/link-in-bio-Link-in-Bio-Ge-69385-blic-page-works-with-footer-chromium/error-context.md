# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: link-in-bio.spec.ts >> Link-in-Bio Generator Growth Loop >> generator page renders correctly, saves data, and public page works with footer
- Location: src/e2e/link-in-bio.spec.ts:4:9

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: locator('h1').filter({ hasText: 'Awesome E2E Bakery' })
Expected: visible
Timeout: 5000ms
Error: element(s) not found

Call log:
  - Expect "toBeVisible" with timeout 5000ms
  - waiting for locator('h1').filter({ hasText: 'Awesome E2E Bakery' })

```

```yaml
- banner:
  - heading "Link-in-Bio Generator 🔗" [level=1]
  - button "Back to Dashboard"
  - text: AC
- main:
  - heading "Profile Details" [level=2]
  - text: Business Name
  - textbox "Business name": Awesome E2E Bakery
  - text: Bio / Tagline
  - textbox "Bio tagline": The best automated cakes in town.
  - text: Theme
  - button "Gradient theme" [pressed]
  - button "Dark theme"
  - button "Light theme"
  - button "Purple theme"
  - heading "Links" [level=2]
  - text: Link 1
  - button "Remove"
  - textbox "Link 1 title":
    - /placeholder: Title (e.g. Visit my Shop)
    - text: Visit My Store
  - textbox "Link 1 URL":
    - /placeholder: URL (e.g. https://...)
    - text: /website-builder
  - text: Link 2
  - button "Remove"
  - textbox "Link 2 title":
    - /placeholder: Title (e.g. Visit my Shop)
    - text: Book an Appointment
  - textbox "Link 2 URL":
    - /placeholder: URL (e.g. https://...)
    - text: /booking
  - button "+ Add Another Link"
  - heading "Publish & Share" [level=2]
  - button "Copy Link-in-Bio URL"
  - paragraph: Add this link to your Instagram, TikTok, or Twitter profile.
  - text: ✨
  - heading "My Store" [level=1]
  - paragraph: Welcome to my storefront!
  - link "Visit My Store":
    - /url: /website-builder
  - link "Book an Appointment":
    - /url: /booking
  - link "⚡ Powered by OHC":
    - /url: /onboarding?ref=my-store
- button "Help":
  - img
- button "Open help chat": ✨Ask anything
```

# Test source

```ts
  1  | import { test, expect } from '@playwright/test';
  2  |
  3  | test.describe('Link-in-Bio Generator Growth Loop', () => {
  4  |     test('generator page renders correctly, saves data, and public page works with footer', async ({ page }) => {
  5  |         // 1. Set some initial local storage state to act as a logged-in user
  6  |         await page.goto('/dashboard');
  7  |         await page.evaluate(() => {
  8  |             localStorage.setItem('tenant', 'e2e-bakery');
  9  |         });
  10 |
  11 |         // 2. Go to the Link-in-Bio Generator page
  12 |         await page.goto('/link-in-bio-generator');
  13 |
  14 |         // Check the page header
  15 |         await expect(page.locator('h1', { hasText: 'Link-in-Bio Generator' })).toBeVisible();
  16 |
  17 |         // 3. Configure the bio page
  18 |         const businessNameInputs = page.locator('input');
  19 |         // The first input is usually the store name based on layout
  20 |         await businessNameInputs.first().fill('Awesome E2E Bakery');
  21 |
  22 |         const bioTextarea = page.locator('textarea');
  23 |         await bioTextarea.fill('The best automated cakes in town.');
  24 |
  25 |         // 4. Verify preview updates in real-time
> 26 |         await expect(page.locator('h1', { hasText: 'Awesome E2E Bakery' })).toBeVisible();
     |                                                                             ^ Error: expect(locator).toBeVisible() failed
  27 |         await expect(page.locator('p', { hasText: 'The best automated cakes in town.' })).toBeVisible();
  28 |
  29 |         // Check the "Powered by OHC" footer in the live preview
  30 |         const previewFooterLink = page.locator('a', { hasText: 'Powered by OHC' });
  31 |         await expect(previewFooterLink).toBeVisible();
  32 |         await expect(previewFooterLink).toHaveAttribute('href', /^https:\/\/ohc\.store\/join\?ref=e2e-bakery/);
  33 |
  34 |         // Wait a moment for the useEffect to save to localStorage
  35 |         await page.waitForTimeout(500);
  36 |
  37 |         // 5. Navigate to the generated public page
  38 |         await page.goto('/bio/e2e-bakery');
  39 |
  40 |         // Verify the public page loaded the saved data
  41 |         await expect(page.locator('h1', { hasText: 'Awesome E2E Bakery' })).toBeVisible();
  42 |         await expect(page.locator('p', { hasText: 'The best automated cakes in town.' })).toBeVisible();
  43 |
  44 |         // Verify the viral footer exists on the public page
  45 |         const publicFooterLink = page.locator('a', { hasText: 'Powered by OHC' });
  46 |         await expect(publicFooterLink).toBeVisible();
  47 |         await expect(publicFooterLink).toHaveAttribute('href', 'https://ohc.store/join?ref=e2e-bakery');
  48 |     });
  49 |
  50 |     test('Dashboard contains link to Link-in-Bio generator', async ({ page }) => {
  51 |         await page.goto('/dashboard');
  52 |
  53 |         // Find the link to create a link in bio page
  54 |         const linkInBioButton = page.locator('a[href="/link-in-bio-generator"]');
  55 |         await expect(linkInBioButton).toBeVisible();
  56 |         await expect(linkInBioButton).toContainText('Create Link-in-Bio Page');
  57 |     });
  58 | });
  59 |
```