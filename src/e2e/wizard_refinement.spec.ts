import { test, expect } from '@playwright/test';

test.describe('Wizard Refinement E2E', () => {
  test('keeps the setup flow plain-language', async ({ page }) => {
    await page.goto('/src/ui/setup.html');
    await expect(page.getByRole('heading', { name: "How do you work?" })).toBeVisible();
    await page.getByText('Local Service').click();
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.getByRole('heading', { name: "What's your category?" })).toBeVisible();
    // Validate plain language explanation exists
    await expect(page.getByText('e.g. Baking, Plumbing, Design')).toBeVisible();
  });

  test('exposes AI helper correctly', async ({ page }) => {
    // Navigate to step 4 to test AI helper wording
    await page.goto('/src/ui/setup.html');

    // Step 1
    await page.getByText('Local Service').click();
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 2
    await page.getByPlaceholder("e.g. Graphic Design").fill("Test");
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 3
    await page.getByPlaceholder("e.g. Maya's Bakery").fill("Test");
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: 'Set up your Assistant' })).toBeVisible();
    await expect(page.getByText('Give your AI assistant a name and tone.')).toBeVisible();
  });

  test('settings back button works', async ({ page }) => {
    await page.goto('/src/ui/setup.html');

    // Step 1
    await page.getByText('Local Service').click();
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: "What's your category?" })).toBeVisible();

    // Hit back button
    await page.getByRole('button', { name: 'Back' }).click();
    await expect(page.getByRole('heading', { name: "How do you work?" })).toBeVisible();
  });
});
