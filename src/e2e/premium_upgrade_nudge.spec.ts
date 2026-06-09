import { test, expect } from '@playwright/test';

test.describe('Premium Upgrade Nudge', () => {
  test('should display the premium upgrade nudge and allow dismissing', async ({ page }) => {
    // Navigate to a page where the AppShell is rendered (which includes the nudge)
    await page.goto('/dashboard');

    // Wait for the nudge to be visible
    const nudge = page.locator('text="Unlock AI Superpowers"').first();
    await expect(nudge).toBeVisible({ timeout: 10000 });

    // Check for the call to action button
    const cta = page.locator('text="View Pro Features"').first();
    await expect(cta).toBeVisible();
    await expect(cta).toHaveAttribute('href', '/pricing');

    // Test the dismiss functionality
    const dismissButton = page.locator('button[aria-label="Dismiss"]').first();
    await expect(dismissButton).toBeVisible();
    await dismissButton.dispatchEvent('click');

    // Ensure the nudge is no longer visible
    await expect(nudge).not.toBeVisible();
  });
});
