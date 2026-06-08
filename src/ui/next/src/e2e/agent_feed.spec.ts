import { test, expect } from '@playwright/test';

test.describe('Agent Feed (Work Triage)', () => {
    test.beforeEach(async ({ page }) => {
        // Assume test user is logged in based on Playwright global setup
        await page.goto('/work-feed');
    });

    test('should display agent feed items and allow approval', async ({ page, request }) => {
        // 1. Simulate an incoming webhook to create a Work Item
        const res = await request.post('/api/v1/work-feed', {
            data: {
                tenant_id: '00000000-0000-0000-0000-000000000000', // Default fallback
                type_: 'message',
                title: 'Instagram DM: Pricing Inquiry',
                preview: 'Do you have vegan cakes?',
                payload: { source: 'instagram', user: 'customer123' }
            }
        });
        expect(res.ok()).toBeTruthy();

        // 2. Refresh page to see new item
        await page.reload();

        // 3. Verify item is in the feed
        await expect(page.locator('text=Instagram DM: Pricing Inquiry')).toBeVisible();
        await expect(page.locator('text="Do you have vegan cakes?"')).toBeVisible();

        // 4. Verify AI draft is shown
        await expect(page.locator('text=Agent Draft')).toBeVisible();
        await expect(page.locator('text=Drafted response for: Instagram DM: Pricing Inquiry')).toBeVisible();

        // 5. Approve the drafted response
        await page.locator('button:has-text("Approve")').first().click();

        // 6. Verify item is removed from feed
        await expect(page.locator('text=Instagram DM: Pricing Inquiry')).not.toBeVisible();
    });
});
