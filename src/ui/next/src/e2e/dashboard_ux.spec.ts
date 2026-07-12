import { test, expect } from '../../../../e2e/fixtures';
import { currentAppSmoke } from '../../../../e2e/current_app_smoke';
import { test as base } from '../../../../e2e/fixtures';

test.describe('Dashboard UX', () => {
  test('should display Growth & Virality section with Share Cards link', async ({ page }) => {
    await page.goto('/dashboard');

    // Verify Growth & Virality section
    await expect(page.locator('h2', { hasText: 'Growth & Virality' })).toBeVisible();

    // Verify Social Share Cards link
    const shareCardsLink = page.locator('a[href="/share-cards"]');
    await expect(shareCardsLink).toBeVisible();
    await expect(shareCardsLink).toContainText('Social Share Cards');
  });

  test('should have 3 columns in Growth & Virality grid', async ({ page }) => {
    await page.goto('/dashboard');

    const shareCardsLink = page.locator('a[href="/share-cards"]');
    const gridContainer = shareCardsLink.locator('..');

    await expect(gridContainer).toHaveClass(/grid-cols-1/);
    await expect(gridContainer).toHaveClass(/lg:grid-cols-3/);
  });

  test('should verify Social Share Cards card description', async ({ page }) => {
    await page.goto('/dashboard');

    const shareCardsLink = page.locator('a[href="/share-cards"]');
    await expect(shareCardsLink.locator('p', { hasText: 'Generate Share Cards to promote your brand on social media.' })).toBeVisible();
  });

  test('should verify Social Share Cards badge icon and label', async ({ page }) => {
    await page.goto('/dashboard');

    const shareCardsLink = page.locator('a[href="/share-cards"]');
    await expect(shareCardsLink.locator('div.rounded-full', { hasText: '🎴' })).toBeVisible();
    await expect(shareCardsLink.locator('div.rounded-full', { hasText: 'Cards' })).toBeVisible();
  });

  test('should verify all links in Growth & Virality section are present', async ({ page }) => {
    await page.goto('/dashboard');

    await expect(page.locator('a[href="/referrals"]').first()).toBeVisible();
    await expect(page.locator('a[href="/milestone-alerts"]')).toBeVisible();
    await expect(page.locator('a[href="/milestone-alerts"] h3')).toHaveText('Milestones');
    await expect(page.locator('a[href="/milestone-alerts"] div.text-2xl', { hasText: '🏆' })).toHaveAttribute('aria-hidden', 'true');
    await expect(page.locator('a[href="/share-cards"]')).toBeVisible();
  });
});

test('current app smoke test', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'dashboard_ux');
});
