import { test, expect } from './fixtures';

test.describe('Changelog Page', () => {
  test('should load the changelog page and show title', async ({ page }) => {
    await page.goto('/changelog');

    const title = page.getByTestId('changelog-title');
    await expect(title).toBeVisible();
    await expect(title).toHaveText(/Release Notes & Changelog/i);
  });

  test('should display at least one version section or the empty state', async ({ page }) => {
    await page.goto('/changelog');

    // Wait for the loading spinner to go away if any
    await page.waitForSelector('.animate-spin', { state: 'hidden', timeout: 5000 }).catch(() => {});

    // It should either show a version header or the empty state
    const hasVersionHeader = await page.locator('h2').count() > 0;
    const hasEmptyState = await page.getByText(/No changelog available./i).count() > 0;

    expect(hasVersionHeader || hasEmptyState).toBeTruthy();
  });

  test('should link to the full technical changelog', async ({ page }) => {
    await page.goto('/changelog');

    const link = page.locator('a[href="https://onehumancorp.com/changelog"]');
    await expect(link).toBeVisible();
    await expect(link).toContainText('Read the full technical changelog on our website');
  });

  test('should render properly on mobile viewport', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/changelog');

    await expect(page.getByTestId('changelog-title')).toBeVisible();

    // Ensure the external link doesn't break the layout
    const link = page.locator('a[href="https://onehumancorp.com/changelog"]');
    await expect(link).toBeVisible();
  });
});
