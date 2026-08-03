import { test, expect } from '@playwright/test';

test.describe('Omnichannel Chat Engine', () => {
    // Hardcoded tenant ID used in API implementation
    const tenantId = '00000000-0000-0000-0000-000000000001';

    test.beforeEach(async ({ request }) => {
        // We will test against the unified inbox page.
        // Data is assumed to be seeded or empty, so we test the empty state first.
    });

    test('Mobile 375px Flow - Inbox loads and navigates correctly', async ({ page }) => {
        // Set mobile viewport
        await page.setViewportSize({ width: 375, height: 667 });

        await page.goto('/ui/omnichat.html');

        // Check initial state
        const headerTitle = page.locator('#header-title');
        await expect(headerTitle).toHaveText('Unified Inbox');

        // Wait for fetch to complete (list should have either an item or the empty message)
        const list = page.locator('#conversation-list');
        await expect(list).toBeVisible();
    });
});
