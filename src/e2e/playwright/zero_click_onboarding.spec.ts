import { test, expect } from '@playwright/test';

test.describe('Zero-Click Onboarding Flow', () => {
  test.use({ viewport: { width: 375, height: 667 } }); // strictly mobile viewport

  test('should complete the zero-click onboarding flow on mobile', async ({ page }) => {
    // Navigate to onboarding page
    await page.goto('/onboarding/zero-click');
    await expect(page).toHaveTitle(/OneHumanCorp|OHC/);

    // Initial Screen
    await expect(page.locator('h1', { hasText: 'Zero-Click Business Generator' })).toBeVisible({ timeout: 15000 });

    // Check if the chat assistant loaded
    await expect(page.getByText('OHC Setup Assistant')).toBeVisible();

    // The user input should be visible
    const input = page.getByPlaceholder('e.g. I am a home baker in Austin selling custom vegan cakes.');
    await expect(input).toBeVisible();

    // Type into the input
    await input.fill('I am a baker in Austin selling custom cakes');

    // Click the submit button
    const submitBtn = page.locator('button[type="submit"]');
    await submitBtn.click();

    // Verify provisioning UI
    await expect(page.getByText('Building Your Business...')).toBeVisible({ timeout: 10000 });
    await expect(page.getByText('Provisioning workspace, products, and agents.')).toBeVisible();

    // Success screen
    await expect(page.locator('h2', { hasText: 'Your business is live!' })).toBeVisible({ timeout: 30000 });

    // Verify the iframe is visible
    const iframe = page.locator('iframe[title="Live Storefront Preview"]');
    await expect(iframe).toBeVisible();

    // Verify action buttons
    await expect(page.locator('button', { hasText: '🚀 Launch My Store' })).toBeVisible();
    await expect(page.locator('button', { hasText: '🐦 Share on X (Twitter)' })).toBeVisible();
  });
});
