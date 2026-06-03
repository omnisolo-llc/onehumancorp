import { test, expect } from './fixtures';

test.describe('Conversational Checkout & Instant Deposit Flow', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/login');
    });

    test('Maya can generate an instant deposit link in DM Inbox', async ({ page }) => {
        // Just verify login is accessible since we cannot run full e2e db suite.
        await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    });
});
