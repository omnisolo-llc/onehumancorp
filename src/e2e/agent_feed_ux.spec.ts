import { test, expect } from '@playwright/test';
import { adminUser, loginAs } from './fixtures';

test.describe('Agent Feed UI Glassmorphism', () => {
  test('Agent Feed section is visible and styled correctly', async ({ page }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard.html');

    // Wait for the Unified Agent Feed section to load
    const feedSection = page.locator('#unified-agent-feed-section');
    await expect(feedSection).toBeVisible();

    // Verify it contains the Command Center title
    await expect(feedSection.locator('h2', { hasText: 'Command Center' })).toBeVisible();

    // Check for feed items
    const feedItems = feedSection.locator('.triage-item');
    // Ensure there's at least one feed item (seeded data)
    expect(await feedItems.count()).toBeGreaterThan(0);

    // Assert visual styling
    const firstItem = feedItems.first();
    await expect(firstItem).toHaveClass(/glassmorphism/);
  });
});
