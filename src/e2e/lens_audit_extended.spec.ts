import { test, expect } from '@playwright/test';

const CUJ_PATHS = [
    '/dashboard',
    '/login',
    '/signup',
    '/pricing',
    '/my-plan',
    '/agents',
    '/diagnostics',
    '/services',
    '/scaling',
    '/website-builder',
    '/settings',
    '/checkout',
    '/users',
    '/referrals',
    '/inbox',
    '/meetings',
];

test.describe('Lens Audit Extended Deep Crawl', () => {
    test.beforeEach(async ({ page }) => {
        // Set fallback state
        await page.goto('/');
    });

    for (const path of CUJ_PATHS) {
        test(`Crawl and verify no runtime errors on ${path}`, async ({ page }) => {
            const errors: string[] = [];
            page.on('pageerror', error => errors.push(error.message));

            await page.goto(path);
            await page.waitForTimeout(500); // Allow JS to execute

            expect(errors.length).toBe(0);

            // Verify screen displays
            const count = await page.locator('.screen').count();
            if (count > 0) {
                // At least one screen should be visible
                const visibleScreens = await page.locator('.screen').filter({ visible: true }).count();
                expect(visibleScreens).toBeGreaterThanOrEqual(1);
            }
        });
    }

    test('Verify full navigation loop', async ({ page }) => {
        await page.goto('/login');
        await page.click('button:has-text("Sign Up")');
        await expect(page.locator('h1:has-text("Create an account")')).toBeVisible();

        await page.click('button:has-text("Start Business Setup")');
        await expect(page.locator('h1:has-text("Your business, live in minutes.")')).toBeVisible();
    });
});
