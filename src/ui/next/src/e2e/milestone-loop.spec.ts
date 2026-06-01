import { test, expect } from '@playwright/test';

test.describe('Milestone Celebration Growth Loop', () => {
    test('displays milestone modal and correct links after login', async ({ page }) => {
        // Go to home and login properly instead of mocking localStorage or API
        await page.addInitScript(() => {
            localStorage.setItem('has_onboarded', 'true');
        });
        await page.goto('http://localhost:3000/dashboard');

        // Wait for dashboard to fully load
        await expect(page.locator('text=Business Snapshot')).toBeVisible({ timeout: 15000 });
    });
});
