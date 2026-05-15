import { test, expect } from '@playwright/test';

test.describe('Swarm Observability Panel', () => {

    test.beforeEach(async ({ page }) => {
        await page.goto('/login');
        await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
        await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
        await page.locator('button:has-text("Login")').filter({ visible: true }).first().click();
        await expect(page.locator('text="Welcome back, Human."')).toBeVisible();
    });

    test('Agent activity panel is visible and initially shows no recent activity', async ({ page }) => {
        await expect(page.locator('text="Agent Activity"')).toBeVisible();
        await expect(page.locator('#agent-activity-feed')).toContainText('No recent activity.');
    });

    test('Simulate Order button adds activity items', async ({ page }) => {
        await page.click('button:has-text("Simulate Order")');
        await expect(page.locator('.activity-item')).toHaveCount(4, { timeout: 5000 });
    });

    test('Support agent plain language message is displayed', async ({ page }) => {
        await page.click('button:has-text("Simulate Order")');
        await expect(page.locator('text="✅ Your Support Agent replied to 3 customers"')).toBeVisible({ timeout: 5000 });
    });

    test('Order manager plain language message is displayed', async ({ page }) => {
        await page.click('button:has-text("Simulate Order")');
        await expect(page.locator('text="📦 Order Manager updated stock for 12 items"')).toBeVisible({ timeout: 5000 });
    });

    test('System state transition messages are displayed', async ({ page }) => {
        await page.click('button:has-text("Simulate Order")');
        await expect(page.locator('text="Operations processed OrderReceived"')).toBeVisible({ timeout: 5000 });
        await expect(page.locator('text="Customer Success drafted confirmation"')).toBeVisible({ timeout: 5000 });
    });

});
