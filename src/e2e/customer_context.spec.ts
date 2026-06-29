import { test, expect } from './fixtures';

test.describe('Omnichannel Unified Customer Memory Graph UI', () => {
  // Use adminPage fixture which handles authentication
  test('displays customer context in the Ambassador Reply Card', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Wait for the unified agent feed to load
    await expect(page.locator('#unified-agent-feed-section')).toBeVisible();

    // Verify the ambassador reply card is rendered (seeded in e2e-seed.sql)
    const card = page.locator('[data-testid="ambassador-reply-card"]').first();
    await expect(card).toBeVisible();

    // Check if the context section is rendered
    await expect(card.locator('text=Customer Context')).toBeVisible();

    // Check past orders is rendered properly
    await expect(card.locator('text=Returning Customer (2 past orders).')).toBeVisible();

    // Check context is rendered properly
    await expect(card.locator('text=Customer prefers vegan options.')).toBeVisible();
  });
});
