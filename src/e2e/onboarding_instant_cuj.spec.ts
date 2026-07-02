import { test, expect } from '@playwright/test';

test.describe('Instant Setup CUJ', () => {

  test.beforeEach(async ({ page }) => {
    // Clean up local storage to ensure fresh start
    await page.addInitScript(() => window.localStorage.clear());
    // Set a known viewport for mobile tests (375px first as per requirements)
    await page.setViewportSize({ width: 375, height: 812 });
  });

  test('Persona: Maya (Home Baker) completes the Zero-Click Instant Onboarding', async ({ page }) => {


    await page.goto('/setup.html');


    // Verify Initial Screen
    await expect(page.locator('h1', { hasText: 'Tell us about your business' })).toBeVisible({ timeout: 15000 });

    // 1. Click "Instant Build"
    await page.getByRole('button', { name: 'Instant Build' }).click();

    // 2. Verify we are in the instant step
    await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();

    // 3. Fill in the description
    const instantInput = page.locator('#instant-bio');
    await expect(instantInput).toBeVisible();
    await instantInput.fill('I make custom vegan cakes in Austin. I need a website and a way to take bookings.');

    const generateBtn = page.getByTestId('generate-storefront-btn');
    await expect(generateBtn).toBeEnabled();

    // Test the bug fix by navigating back and forward to ensure text is preserved
    await page.getByRole('button', { name: 'Back' }).click();
    await expect(page.locator('h1', { hasText: 'Tell us about your business' })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: 'Instant Build' }).click();
    await expect(instantInput).toHaveValue('I make custom vegan cakes in Austin. I need a website and a way to take bookings.');
    await expect(generateBtn).toBeEnabled();

    // 4. Click generate
    await generateBtn.click();

    // 5. Verify loading texts (animation progress)
    const btnText = await generateBtn.innerText();
    expect(btnText).toContain('Analyzing request...');

    // Check if the text changes to the next one
    await expect(generateBtn).toContainText('Designing storefront...', { timeout: 4000 });

    await expect(page).toHaveURL(/.*success.html/, { timeout: 60000 });
  });
});
