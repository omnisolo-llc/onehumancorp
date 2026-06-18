import { test, expect } from '@playwright/test';

test.describe('Zero Click Builder', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should generate a business successfully on mobile and redirect to feed', async ({ page }) => {
    // We navigate directly to the zero-click-builder page.
    await page.goto('http://localhost:3000/zero-click-builder');

    // Make sure the title is visible
    await expect(page.getByText('Zero-Click Business Generator')).toBeVisible();

    // Fill in the prompt
    await page.fill('textarea[id="prompt"]', 'I sell custom sneakers in Austin via delivery.');

    // Click the generate button
    await page.click('button:has-text("Generate My Business")');

    // Should see loading steps
    await expect(page.getByText('Analyzing your business...')).toBeVisible();

    // Increase timeout since backend could take time, or interval takes a bit
    await expect(page.getByText('Your business is live!')).toBeVisible({ timeout: 10000 });

    // Ensure the Launch My Store button exists and routes to /feed
    const launchButton = page.getByRole('button', { name: 'Launch My Store' });
    await expect(launchButton).toBeVisible();
    await launchButton.click();

    // Assert that we are redirected to the Unified Feed
    await expect(page).toHaveURL(/.*\/feed/);
  });
});
