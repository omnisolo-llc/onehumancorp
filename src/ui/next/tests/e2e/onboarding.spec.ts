import { test, expect } from '@playwright/test';

test.describe('Onboarding flows', () => {
  test('Zero-Click Onboarding flow interactive steps', async ({ page }) => {
    // 1. Start at the zero-click onboarding page
    await page.goto('http://localhost:3000/onboarding/zero-click');
    await expect(page).toHaveTitle(/OmniSolo/);

    // 2. Verify initial rendering and text
    await expect(page.locator('text=Tell us about your business')).toBeVisible();
    await expect(page.getByPlaceholder('e.g. I am a home baker in Austin selling custom vegan cakes.')).toBeVisible();

    // 3. Fill in the input field
    const input = page.getByPlaceholder('e.g. I am a home baker in Austin selling custom vegan cakes.');
    await input.fill('I sell custom sneakers in New York.');

    // 4. Submit the form
    const submitBtn = page.getByRole('button', { name: /Send|Generate/i }); // Fallback regex in case button name differs
    await expect(submitBtn).toBeEnabled();
    await submitBtn.click();

    // 5. Verify the transition state (loading or result)
    // Wait for the resulting "Your business is live!" or equivalent transition
    await expect(page.locator('text=Your business is live!')).toBeVisible({ timeout: 15000 });

    // 6. Verify final actions
    await expect(page.getByRole('button', { name: /Launch My Store/i })).toBeVisible();
    await expect(page.getByRole('button', { name: /Share on X/i })).toBeVisible();
  });
});
