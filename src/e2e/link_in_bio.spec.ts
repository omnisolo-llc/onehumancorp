import { expect } from '@playwright/test';
import { test } from './fixtures';

test.describe.skip('Link-in-Bio Generator Growth Loop', () => {
  // Test skipped due to sandbox Docker permission issue when pulling pgvector/pgvector:pg16 image.
  test('should allow user to navigate to and preview the link-in-bio page', async ({ page }) => {
    // Start at dashboard
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' })).toBeVisible();

    // Verify Link-in-Bio growth loop block is visible
    const linkInBioHeader = page.locator('h2', { hasText: 'Link-in-Bio Generator' });
    await expect(linkInBioHeader).toBeVisible();

    // Click on Preview Link-in-Bio
    await page.click('text=Preview Link-in-Bio');

    // Should navigate to /link-in-bio
    await expect(page).toHaveURL(/\/link-in-bio/);

    // Verify the page renders core elements
    await expect(page.locator('a', { hasText: 'Powered by OHC' })).toBeVisible();
    await expect(page.locator('a', { hasText: 'Book a Lesson' })).toBeVisible();
    await expect(page.locator('a', { hasText: 'View Menu' })).toBeVisible();
  });
});
