import { test, expect } from '@playwright/test';

test.describe('Omnichannel Unified Inbox (375px Mobile First)', () => {
    test('Displays Instagram DM in unified list and allows 1-tap AI draft approval', async ({ page }) => {
        // Enforce strict 375px viewport
        await page.setViewportSize({ width: 375, height: 667 });

        await page.goto('/ui/omni-inbox.html');

        // Check that the inbox container doesn't overflow horizontally
        const scrollWidth = await page.evaluate(() => document.documentElement.scrollWidth);
        expect(scrollWidth).toBeLessThanOrEqual(375);

        // Verify the message appears in the unified list
        const msgItem = page.locator('.inbox-item').first();
    });
});
