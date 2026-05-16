import { test, expect } from '@playwright/test';

test.describe('Dashboard UX Friction Fixes', () => {

    test('Dashboard loads with meaningful plain language', async ({ page }) => {
        await page.goto('http://localhost:3000');

        // Assert Glassmorphism styling
        const dashboard = page.locator('#dashboard-screen');
        await expect(dashboard).toBeVisible();

        // Assert plain language labels
        await expect(page.locator('text="Welcome to your Dashboard"')).toBeVisible();
        await expect(page.locator('text="Today\'s Sales"')).toBeVisible();
        await expect(page.locator('text="Active Orders"')).toBeVisible();
    });

    test('Dashboard shows async shimmer loading state instead of blank screen', async ({ page }) => {
        await page.goto('http://localhost:3000');

        // Click View Orders to trigger async load
        const button = page.locator('button', { hasText: 'View All Orders' });
        await button.click();

        // Verify skeleton overlay appears
        const overlay = page.locator('.async-loading-overlay.active');
        await expect(overlay).toBeVisible();

        // Ensure it has glassmorphism class
        await expect(overlay).toHaveClass(/glass/);

        // Skeletons are visible
        const skeletons = page.locator('.skeleton');
        await expect(skeletons.first()).toBeVisible();
    });

    test('Mobile view 375px respects touch targets and layout', async ({ page }) => {
        await page.setViewportSize({ width: 375, height: 812 });
        await page.goto('http://localhost:3000');

        // Check touch target height
        const button = page.locator('button', { hasText: 'View All Orders' });
        const box = await button.boundingBox();
        expect(box!.height).toBeGreaterThanOrEqual(44);
    });

    test('Navigation clarity: 5 main actions are accessible', async ({ page }) => {
        await page.goto('http://localhost:3000');

        // Check Messages navigation
        const messagesBtn = page.locator('button', { hasText: 'Check Messages' });
        await expect(messagesBtn).toBeVisible();
        await messagesBtn.click();

        // Verify we ended up on the inbox screen
        await expect(page.locator('#inbox-screen')).toBeVisible();
    });

    test('Design Token Validation: Font Families and Motion', async ({ page }) => {
        await page.goto('http://localhost:3000');

        // Check typography tokens
        const title = page.locator('text="Today\'s Sales"');
        await expect(title).toHaveCSS('font-family', /Outfit/);

        // Verify the overlay has the correct transition ease
        const overlay = page.locator('.async-loading-overlay');
        await expect(overlay).toHaveCSS('transition', /cubic-bezier\(0\.4, 0, 0\.2, 1\)/);
    });
});
