import { test, expect } from '@playwright/test';

test.describe('Onboarding flows', () => {
  test('Zero-Click Onboarding flow interactive steps', async ({ page }) => {
    // 1. Start at the zero-click onboarding page
    await page.goto('/onboarding/zero-click');
    await expect(page).toHaveTitle(/OmniSolo/);

    // 2. Verify initial rendering and text
    await expect(page.locator('text=Tell us about your business')).toBeVisible();
    await expect(page.getByPlaceholder('e.g. I am a home baker in Austin selling custom vegan cakes.')).toBeVisible();

    // 3. Fill in the input field
    const input = page.getByPlaceholder('e.g. I am a home baker in Austin selling custom vegan cakes.');
    await input.fill('I sell custom sneakers in New York.');

    // 4. Submit the form
    const submitBtn = page.getByRole('button', { name: 'Send message', exact: true });
    await expect(submitBtn).toBeEnabled();
    await submitBtn.click();

    // Chat prepares a review. Saving and completion require separate owner actions.
    await page.getByRole('button', { name: /Approve.*Prepare Workspace/ }).click();
    await expect(page.getByText('Your workspace is prepared')).toBeVisible();
    await expect(page.getByText('Setup complete')).toHaveCount(0);
    await page.getByRole('button', { name: /Launch My Store/i }).click();
    await expect(page.getByText('Setup complete')).toBeVisible({ timeout: 15000 });
    await expect(page.getByRole('button', { name: /Go to dashboard/i })).toBeVisible();
    await expect(page.getByRole('button', { name: /Share on X/i })).toBeVisible();
  });
});
