import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Viral Community Goal Widget Generator', () => {
  test('navigates to generator and verifies basic functionality', async ({ adminPage: page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard.html');

    // Click on the Community Goal Widget link
    await page.click('#viral-community-goal-widget-link');

    // Verify navigation
    await expect(page).toHaveURL(/.*viral-community-goal-widget\.html/);

    // Verify main elements exist
    await expect(page.locator('h1')).toContainText('Community Goal Widget');
    await expect(page.locator('#goal-target')).toBeVisible();
    await expect(page.locator('#goal-reward')).toBeVisible();

    // Test generation logic
    await page.fill('#goal-target', '1500');
    await page.fill('#goal-reward', 'Free T-Shirt');
    await page.click('#generate-btn');

    // Verify result area becomes visible
    await expect(page.locator('#result-area')).toBeVisible();
    await expect(page.locator('#embed-code')).toContainText('target=1500');
    await expect(page.locator('#embed-code')).toContainText('reward=Free%20T-Shirt');
  });
});
