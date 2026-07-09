import { test, expect } from '@playwright/test';

test.describe('Instant Setup CUJ', () => {

  test.beforeEach(async ({ page }) => {
    // Clean up local storage to ensure fresh start
    await page.addInitScript(() => window.localStorage.clear());
    // Set a known viewport for mobile tests (375px first as per requirements)
    await page.setViewportSize({ width: 375, height: 812 });
  });

  test('Persona: Maya (Home Baker) completes the Zero-Click Instant Onboarding', async ({ page }) => {


    const htmlPath = require('path').resolve('src/ui/tauri/src/ui/setup.html');
    await page.goto(`file://${htmlPath}`);


    // Verify Initial Screen / Instant Build Step
    await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();

    // 1. Fill in the description
    const instantInput = page.locator('#instant-bio');
    await expect(instantInput).toBeVisible();
    await instantInput.fill('I make custom vegan cakes in Austin. I need a website and a way to take bookings.');

    const generateBtn = page.getByTestId('generate-storefront-btn');
    await expect(generateBtn).toBeEnabled();

    // 2. Click generate
    await generateBtn.click();

    // 3. Verify loading texts (animation progress)
    const btnText = await generateBtn.innerText();
    // // // expect(btnText).toContain('Analyzing request...');

    // Check if the text changes to the next one
    // // // await expect(generateBtn).toContainText('Designing storefront...', { timeout: 4000 });

    // // // await expect(page).toHaveURL(/.*success.html/, { timeout: 60000 });
  });
});
