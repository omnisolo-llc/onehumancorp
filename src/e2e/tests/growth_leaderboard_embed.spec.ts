import { test, expect } from '@playwright/test';
import { adminPage } from '../fixtures';

test.describe('Growth Leaderboard Embed', () => {
  test('User can view and copy the leaderboard embed, and the embed renders correctly', async ({ page }) => {
    // Navigate to the referral leaderboard generator UI
    // We are mocking localStorage before navigation since the script relies on it.
    await page.addInitScript(() => {
      window.localStorage.setItem('tenant', 'e2e-tenant');
      window.localStorage.setItem('tenant_id', 'e2e-tenant');
    });

    await page.goto('/referral-leaderboard-generator.html');

    // Wait for the preview container to populate (mock API handles it or real API from backend)
    // We expect the backend to return leaderboard data or an empty state.
    // The "Embed Code" section should become visible.
    await expect(page.locator('#embed-section')).toBeVisible({ timeout: 10000 });

    // The copy button should be available
    const copyButton = page.locator('button', { hasText: 'Copy Embed Code' }).first();
    await expect(copyButton).toBeEnabled();

    // Try clicking copy
    await copyButton.click();
    await expect(page.locator('button', { hasText: 'Copied!' })).toBeVisible();

    // Verify the embed script actually points to the correct widget endpoint
    const codeBlock = page.locator('#embed-code');
    const embedText = await codeBlock.innerText();
    expect(embedText).toContain('/api/v1/growth/embed/widget?type=leaderboard&tenant=e2e-tenant');

    // Now test the actual backend endpoint directly to ensure it renders the HTML
    await page.goto('/api/v1/growth/embed/widget?type=leaderboard&tenant_id=e2e-tenant');

    // Check for the rendered HTML
    await expect(page.locator('h3', { hasText: 'Top Referrers' })).toBeVisible();
    await expect(page.locator('text=Powered by OHC')).toBeVisible();
  });
});
