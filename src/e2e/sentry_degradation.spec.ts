import { test, expect } from './fixtures';

test.describe('🛡️ Sentry: Graceful Degradation', () => {
    test('Verify cached data display when backend returns 500/503', async ({ page }) => {
        await page.goto('/dashboard/services');
        await page.route('**/api/services', async route => {
            await route.fulfill({ status: 503, body: 'Service Unavailable' });
        });
        await page.reload();
        await expect(page.getByText('Offline Mode')).toBeVisible();
    });
});
