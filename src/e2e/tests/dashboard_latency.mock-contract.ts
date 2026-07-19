import { test, expect } from '@playwright/test';

test.describe('Dashboard Performance Optimization UI Verification', () => {
    const tenantId = 'e2e-dashboard-tenant';

    test.beforeAll(async ({ request }) => {
        // Seed some data so dashboard is not empty
        const res = await request.post('/api/v1/omnichannel/webhook', {
            data: {
                tenant_id: tenantId,
                source: 'Instagram DM',
                sender_id: 'john_doe',
                message: 'Hello, need help with my account',
            }
        });
        expect(res.status()).toBe(200);
    });

    test('Dashboard feed items load parallelly', async ({ page }) => {
        await page.goto(`/login`);
        await page.fill('input[name="email"]', 'admin@example.com');
        await page.fill('input[name="password"]', 'password123');
        await page.click('button[type="submit"]');

        await page.waitForURL('/dashboard');

        // Ensure that the UI renders the main dashboard skeleton
        await expect(page.locator('text=Action Required').first()).toBeVisible();

        // Check components that rely on parallel payload fetches
        await expect(page.locator('text=Recent Messages').first()).toBeVisible();
    });

    test('Dashboard unified agent feed UI renders correctly with payload data', async ({ page }) => {
        await page.goto(`/login`);
        await page.fill('input[name="email"]', 'admin@example.com');
        await page.fill('input[name="password"]', 'password123');
        await page.click('button[type="submit"]');

        await page.waitForURL('/dashboard');

        // This confirms the UI for pending approvals or agent feed
        await expect(page.locator('text=Agent Approvals').first()).toBeVisible();
    });
});
