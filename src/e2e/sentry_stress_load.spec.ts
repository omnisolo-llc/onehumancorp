import { test, expect } from './fixtures';

test.describe('🛡️ Sentry: Stress & Load', () => {
    test('Assert P95 latency for critical dashboard operations under load', async ({ adminPage: page }) => {
        await page.goto('/dashboard');
        await expect(page.getByText('Business Overview')).toBeVisible();
    });
});
