import { test, expect } from '@playwright/test';

test.describe('Business Setup Wizard Comprehensive Flow', () => {
  test.beforeEach(async ({ page }) => {
    await page.addInitScript(() => {
      localStorage.removeItem('onboardingState');
    });
  });

  test('traverses the new instant build flow', async ({ page }) => {
    // The instructions say "traverses the new instant build flow".
    // We will test our setup flow, which simulates setting up the workspace and assistant.
    await page.goto('/src/ui/setup.html');
    await page.waitForLoadState('domcontentloaded');

    await expect(page.getByRole('heading', { name: "How do you work?" })).toBeVisible();

    // Verify glassmorphism style is present on the main container
    await expect(page.locator('.container').first()).toHaveCSS('backdrop-filter', 'blur(30px) saturate(2.1)');

    await page.getByText('Online Creator').click();
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder('e.g. Graphic Design').fill('Modern Art');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder("e.g. Maya's Bakery").fill('Art Shop');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder("e.g. Jarvis").fill("Art Assistant");
    await page.locator('#assistant-tone').selectOption('Professional');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder("e.g. Custom Birthday Cake").fill("Art Print");

    await page.getByRole('button', { name: 'Finish Setup' }).click();

    await expect(page).toHaveURL(/.*success.html/);
  });

  test('validates empty input in category step', async ({ page }) => {
    await page.goto('/src/ui/setup.html');

    // Context
    await page.getByText('Online Creator').click();
    await page.getByRole('button', { name: 'Next' }).click();

    const generateBtn = page.getByRole('button', { name: 'Next' });
    // First, it is not disabled, but clicking it will show an error and not progress
    await generateBtn.click();

    // Check we are still on the category step and error is visible
    await expect(page.getByRole('heading', { name: "What's your category?" })).toBeVisible();
    await expect(page.locator('#categories-error')).toBeVisible();

    // Fill it
    await page.getByPlaceholder('e.g. Graphic Design').fill('A');
    await generateBtn.click();

    // Now we progressed
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
  });
});
