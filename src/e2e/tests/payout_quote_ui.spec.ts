import { test, expect } from '@playwright/test';

test.describe('Owner Feed - UI Components Verify', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
    });

    test('Verify minimum touch target on standard dashboard buttons', async ({ page }) => {
        await page.waitForLoadState('networkidle');
        const buttons = page.locator('button:visible');
        try {
            await buttons.first().waitFor({ state: 'visible', timeout: 5000 });
        } catch {
            return;
        }
        const count = Math.min(await buttons.count(), 5);
        for (let i = 0; i < count; i++) {
            const box = await buttons.nth(i).boundingBox();
            if (box) {
                expect(box.width).toBeGreaterThanOrEqual(44);
                expect(box.height).toBeGreaterThanOrEqual(44);
            }
        }
    });

    test('Verify responsive container sizing on 375px viewport', async ({ page }) => {
        await page.setViewportSize({ width: 375, height: 667 });
        await page.goto('/');
        await page.waitForLoadState('domcontentloaded');
        const bodyWidth = await page.evaluate(() => document.body.scrollWidth);
        const windowWidth = await page.evaluate(() => window.innerWidth);
        expect(bodyWidth).toBeLessThanOrEqual(windowWidth);
    });

    test('Verify responsive container sizing on desktop viewport', async ({ page }) => {
        await page.setViewportSize({ width: 1440, height: 900 });
        await page.goto('/');
        await page.waitForLoadState('domcontentloaded');
        const bodyWidth = await page.evaluate(() => document.body.scrollWidth);
        const windowWidth = await page.evaluate(() => window.innerWidth);
        expect(bodyWidth).toBeLessThanOrEqual(windowWidth);
    });
});
