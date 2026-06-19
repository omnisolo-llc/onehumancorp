import { test, expect } from '@playwright/test';

test.describe('Instant Quote Form UI', () => {
    test.beforeEach(async ({ page }) => {
        // Just mock the network route here to simulate empty rules
        await page.route('**/api/quotes/rules*', route => {
            route.fulfill({
                status: 200,
                contentType: 'application/json',
                body: JSON.stringify([])
            });
        });

        await page.goto('/ui/instant-quote.html');
    });

    test('should display empty state notice when no rules are returned', async ({ page }) => {
        const notice = page.locator('.notice');
        await expect(notice).toBeVisible();
        await expect(notice).toHaveText('No quoting rules available. Please configure rules in the OHC dashboard.');
    });

    test('should not show default rush rule checkbox', async ({ page }) => {
        const rushCheckbox = page.locator('input[value="rush"]');
        await expect(rushCheckbox).not.toBeAttached();
    });

    test('should not show default vegan rule checkbox', async ({ page }) => {
        const veganCheckbox = page.locator('input[value="vegan"]');
        await expect(veganCheckbox).not.toBeAttached();
    });

    test('should not show default weekend rule checkbox', async ({ page }) => {
        const weekendCheckbox = page.locator('input[value="weekend"]');
        await expect(weekendCheckbox).not.toBeAttached();
    });

    test('should log error when server fails to load rules', async ({ page }) => {
        const errors: string[] = [];
        page.on('console', msg => {
            if (msg.type() === 'error') {
                errors.push(msg.text());
            }
        });

        await page.route('**/api/quotes/rules*', route => {
            route.fulfill({
                status: 500,
                body: 'Internal Server Error'
            });
        });

        await page.goto('/ui/instant-quote.html');
        const notice = page.locator('.notice');
        await expect(notice).toBeVisible();
        expect(errors.some(e => e.includes('Failed to load rules from server'))).toBeTruthy();
    });
});
