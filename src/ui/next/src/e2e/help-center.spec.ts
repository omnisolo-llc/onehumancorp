import { test, expect } from '@playwright/test';

test('Verify Help Center user journey', async ({ page }) => {
  // Go to help center
  await page.goto('http://localhost:3000/help');

  // Wait for it to load
  await expect(page.locator('h1')).toHaveText('Help Center');

  // Search for an article
  const searchInput = page.locator('input[placeholder="Search for help articles..."]');
  await searchInput.fill('Getting Paid');

  // Click on the matching article
  const articleLink = page.locator('text=Getting Paid');
  await articleLink.click();

  // Verify the article page loaded
  await expect(page.locator('h1')).toHaveText('Getting Paid');
  await expect(page.locator('h2', { hasText: 'Connecting Your Bank Account' })).toBeVisible();

  // Go back to the help center
  const backButton = page.locator('button', { hasText: 'Back to Help Center' });
  await backButton.click();

  // Should be back on the Help Center page
  await expect(page.locator('h1')).toHaveText('Help Center');
});
