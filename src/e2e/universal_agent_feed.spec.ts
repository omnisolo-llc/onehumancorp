import { test, expect } from '@playwright/test';

test.describe('Universal Agent Feed', () => {
  test('user can receive and approve an action card', async ({ page }) => {
    // Navigate to the universal feed page
    await page.goto('/universal-feed');

    // Wait for the feed to load
    await expect(page.locator('h1')).toContainText('Agent Feed');

    // Check that we have drafts
    const cards = page.locator('.w-full.bg-white\\/70');
    await expect(cards).toHaveCount(2);

    // Verify first card content
    const firstCard = cards.first();
    await expect(firstCard).toContainText('Context');
    await expect(firstCard).toContainText('vegan cakes');
    await expect(firstCard).toContainText('Proposed Action');

    // Approve the first card
    await firstCard.locator('button', { hasText: 'Approve & Send' }).click();

    // Verify card is removed
    await expect(cards).toHaveCount(1);

    // Verify second card content
    const secondCard = cards.first();
    await expect(secondCard).toContainText('Handyman Service');

    // Dismiss the second card
    await secondCard.locator('button', { hasText: 'Dismiss' }).click();

    // Verify empty state
    await expect(cards).toHaveCount(0);
    await expect(page.locator('h2')).toContainText('All caught up!');
  });
});
