import { test, expect } from '@playwright/test';

test.describe('Documentation & Help Features', () => {
  test('User can open help widget, search articles, and see tooltips', async ({ page }) => {
    await page.goto('/api/v1/ui/dashboard.html');

    // Verify Help Widget opens
    const helpBtn = page.locator('#ohc-floating-help-btn');
    await expect(helpBtn).toBeVisible();
    await helpBtn.click();

    // Check tabs
    await expect(page.locator('#tab-articles')).toBeVisible();

    // Verify articles are fetched
    const firstArticle = page.locator('#ohc-help-articles-list li a').first();
    await expect(firstArticle).toBeVisible({ timeout: 10000 });

    // Test interactive tours
    const tourTab = page.locator('.ohc-help-tab[data-target="tab-tours"]');
    await tourTab.click();

    // Wait for the tour fetch
    const tourBtn = page.locator('.ohc-tour-card button').first();
    await expect(tourBtn).toBeVisible({ timeout: 10000 });

    // Verify a tooltip (hover over dashboard title)
    const dashboardTitle = page.locator('#dashboard-title');
    await dashboardTitle.hover();
    const tooltip = page.locator('.ohc-tooltip');
    await expect(tooltip).toBeVisible();
    await expect(tooltip).toContainText('This is your dashboard'); // Expected from seed data
  });
});
