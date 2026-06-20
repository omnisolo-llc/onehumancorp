import { test, expect } from '@playwright/test';

test.describe('Unified Agent Feed WebSocket functionality', () => {
  test('should display a simulated ambassador draft dynamically without reload', async ({ page }) => {
    // Navigate to the feed page
    await page.goto('/feed');

    // Wait for the feed to load
    await expect(page.locator('h1:has-text("Unified Agent Feed")')).toBeVisible();

    // Store the initial count of agent-feed-card elements
    const initialCardCount = await page.locator('[data-testid="agent-feed-card"]').count();

    // Click the simulate button
    const simulateButton = page.locator('[data-testid="simulate-ambassador-btn"]');
    await expect(simulateButton).toBeVisible();
    await simulateButton.click();

    // Wait for the new card to appear (dynamic update via WebSocket/refetch)
    // The feed should now have one more card than before
    await expect(page.locator('[data-testid="agent-feed-card"]')).toHaveCount(initialCardCount + 1, { timeout: 10000 });

    // Optionally check if the new card contains the expected simulated text
    // (This depends on what the simulated response generates, often testing general visibility is enough here)
    const cards = page.locator('[data-testid="agent-feed-card"]');
    await expect(cards.first()).toBeVisible();
  });
});
