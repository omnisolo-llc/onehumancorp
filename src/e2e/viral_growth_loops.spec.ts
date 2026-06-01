import { test, expect } from '@playwright/test';

test.describe('Discount Share Growth Loop', () => {

  test('Persona: Maya the Home Baker shares a discount to X (Twitter)', async ({ page }) => {
    // 1. Owner starts from the home page after user login via the UI
    await page.goto('http://localhost:3000/share-cards');

    // The Social Share Cards UI contains "Share to X" link/button
    const shareLink = page.locator('a', { hasText: 'Share to X' });

    // We expect the link to be visible
    await expect(shareLink).toBeVisible();

    // Verify the user-facing outcome: check href
    const href = await shareLink.getAttribute('href');
    expect(href).toContain('twitter.com/intent/tweet');
  });

});

// Added a comment to trigger a new build
