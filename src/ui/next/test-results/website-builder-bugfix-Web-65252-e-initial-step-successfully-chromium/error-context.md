# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: website-builder-bugfix.spec.ts >> Website Builder Tool (E2E Validation) >> renders the initial step successfully
- Location: src/e2e/website-builder-bugfix.spec.ts:4:9

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: locator('h1').filter({ hasText: 'Website Builder' }).or(locator('h1').filter({ hasText: 'What kind of business are you building?' })).first()
Expected: visible
Timeout: 5000ms
Error: element(s) not found

Call log:
  - Expect "toBeVisible" with timeout 5000ms
  - waiting for locator('h1').filter({ hasText: 'Website Builder' }).or(locator('h1').filter({ hasText: 'What kind of business are you building?' })).first()

```

```yaml
- heading "10-Minute Setup Wizard" [level=1]
- heading "Your business, live in minutes." [level=2]
- paragraph: Zero tech skills needed. We do the heavy lifting. Review and add any extra details to help our AI generate the perfect store.
- button "Start My Business"
- button "Instant Build"
- button "Help":
  - img
- button "Open help chat": ✨ Ask anything
- alert
```

# Test source

```ts
  1  | import { test, expect } from '@playwright/test';
  2  |
  3  | test.describe('Website Builder Tool (E2E Validation)', () => {
  4  |     test('renders the initial step successfully', async ({ page }) => {
  5  |         await page.goto('/website-builder');
> 6  |         await expect(page.locator('h1', { hasText: 'Website Builder' }).or(page.locator('h1', { hasText: 'What kind of business are you building?' })).first()).toBeVisible();
     |                                                                                                                                                                 ^ Error: expect(locator).toBeVisible() failed
  7  |     });
  8  |
  9  |     test('can enter business type and advance', async ({ page }) => {
  10 |         await page.goto('/website-builder');
  11 |
  12 |         const typeInput = page.getByPlaceholder('e.g., Coffee Shop, Marketing Agency, Bakery');
  13 |         await expect(typeInput).toBeVisible({ timeout: 60000 });
  14 |         await typeInput.fill('Bakery');
  15 |
  16 |         const nextButton = page.getByRole('button', { name: 'Next' });
  17 |         await expect(nextButton).toBeVisible();
  18 |         await nextButton.click();
  19 |
  20 |         await expect(page.getByText('What is the name of your business?')).toBeVisible();
  21 |     });
  22 |
  23 |     test('can enter business name', async ({ page }) => {
  24 |         await page.goto('/website-builder');
  25 |
  26 |         // Skip first step
  27 |         const typeInput = page.getByPlaceholder('e.g., Coffee Shop, Marketing Agency, Bakery');
  28 |         await expect(typeInput).toBeVisible({ timeout: 60000 });
  29 |         await typeInput.fill('Bakery');
  30 |         await page.getByRole('button', { name: 'Next' }).click();
  31 |
  32 |         // Step 2
  33 |         const nameInput = page.getByPlaceholder('Enter your business name');
  34 |         await expect(nameInput).toBeVisible();
  35 |         await nameInput.fill('Sweet Treats Bakery');
  36 |
  37 |         const nextButton = page.getByRole('button', { name: 'Next' });
  38 |         await nextButton.click();
  39 |
  40 |         await expect(page.getByText('What will you be selling?')).toBeVisible();
  41 |     });
  42 |
  43 |     test('can select selling options', async ({ page }) => {
  44 |         await page.goto('/website-builder');
  45 |
  46 |         // Skip to step 3
  47 |         const typeInput = page.getByPlaceholder('e.g., Coffee Shop, Marketing Agency, Bakery'); await expect(typeInput).toBeVisible({ timeout: 60000 }); await typeInput.fill('Bakery');
  48 |         await page.getByRole('button', { name: 'Next' }).click();
  49 |         await page.getByPlaceholder('Enter your business name').fill('Sweet Treats');
  50 |         await page.getByRole('button', { name: 'Next' }).click();
  51 |
  52 |         // Step 3
  53 |         const physicalProducts = page.getByText('Physical Products');
  54 |         await expect(physicalProducts).toBeVisible();
  55 |         await physicalProducts.click();
  56 |
  57 |         await page.getByRole('button', { name: 'Next' }).click();
  58 |         await expect(page.getByText('Add your first product (optional)')).toBeVisible();
  59 |     });
  60 |
  61 |     test('can skip product addition and reach agent selection', async ({ page }) => {
  62 |         await page.goto('/website-builder');
  63 |
  64 |         // Skip to step 4
  65 |         const typeInput = page.getByPlaceholder('e.g., Coffee Shop, Marketing Agency, Bakery'); await expect(typeInput).toBeVisible({ timeout: 60000 }); await typeInput.fill('Bakery');
  66 |         await page.getByRole('button', { name: 'Next' }).click();
  67 |         await page.getByPlaceholder('Enter your business name').fill('Sweet Treats');
  68 |         await page.getByRole('button', { name: 'Next' }).click();
  69 |         await page.getByText('Physical Products').click();
  70 |         await page.getByRole('button', { name: 'Next' }).click();
  71 |
  72 |         // Step 4: Skip product
  73 |         const skipButton = page.getByRole('button', { name: 'Skip for now' });
  74 |         await expect(skipButton).toBeVisible();
  75 |         await skipButton.click();
  76 |
  77 |         await expect(page.getByText('Pick your AI Agents')).toBeVisible();
  78 |     });
  79 | });
  80 |
```