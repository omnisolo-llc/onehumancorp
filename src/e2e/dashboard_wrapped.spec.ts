import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('dashboard_wrapped_widget', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'dashboard_wrapped_widget');
});

test.describe('Dashboard OHC Wrapped Widget UI', () => {
  test('should display 2026 Wrapped widget with accurate stats and share CTA', async ({ page, loginAs, unlimitedAdminUser }) => {
    // Login
    await loginAs(page, unlimitedAdminUser);

    // Navigate to dashboard
    await page.goto('/dashboard.html');
    await page.waitForLoadState('networkidle');

    // Verify Wrapped widget exists
    // The widget uses display:none initially and needs API response.
    await page.waitForSelector('#wrapped-widget', { state: 'visible' });
    await expect(page.getByRole('heading', { name: /2026 Wrapped/i })).toBeVisible();

    // Verify stats are visible and correct (from fake/mock backend fallback or real seeded)
    await expect(page.locator('#wrapped-sales')).toHaveText('$124,500.00');
    await expect(page.locator('#wrapped-orders')).toHaveText('1420');
    await expect(page.locator('#wrapped-customers')).toHaveText('850');
    await expect(page.locator('#wrapped-ai-hours')).toHaveText('124');

    // Verify WhatsApp share button exists
    const shareBtn = page.locator('#wrapped-share-btn');
    await expect(shareBtn).toBeVisible();
    await expect(shareBtn).toHaveText(/Share 2026 Success/i);
  });
});
