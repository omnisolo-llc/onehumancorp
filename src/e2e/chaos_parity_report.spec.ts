import { test, expect } from '@playwright/test';

test.describe('E2E Chaos Parity Report', () => {
    test.beforeEach(async ({ page }) => {
        // Must start from login without shortcuts per constraints
        await page.goto('/login');
        await page.locator('input[type="email"]').fill('test@example.com');
        await page.locator('input[type="password"]').fill('password123');
        await page.locator('button:has-text("Sign In"), button:has-text("Login")').click();
        await page.waitForURL('**/dashboard**');
    });

    test('should be hidden by default', async ({ page }) => {
        const reportTitle = page.locator('text=Chaos Engineering & Parity Audit');
        await expect(reportTitle).not.toBeVisible();
    });

    test('should open when Chaos Report button is clicked', async ({ page }) => {
        // The button is inside the "Quick Actions" panel
        const chaosBtn = page.locator('button:has-text("Chaos Report")').first();
        await expect(chaosBtn).toBeVisible();
        await chaosBtn.click();

        const reportTitle = page.locator('text=Chaos Engineering & Parity Audit');
        await expect(reportTitle).toBeVisible();
    });

    test('should display key parity metrics from chaos audit', async ({ page }) => {
        const chaosBtn = page.locator('button:has-text("Chaos Report")').first();
        await chaosBtn.click();

        // Check for specific text bound in the Slint default model
        await expect(page.locator('text=Recovery Time (p95)')).toBeVisible();
        await expect(page.locator('text=Sandbox Violations')).toBeVisible();
        await expect(page.locator('text=Sync Lag')).toBeVisible();
        await expect(page.locator('text=1.2s')).toBeVisible();
    });

    test('should display recent hybrid audit experiments', async ({ page }) => {
        const chaosBtn = page.locator('button:has-text("Chaos Report")').first();
        await chaosBtn.click();

        await expect(page.locator('text=Thin Client Fail-Safe')).toBeVisible();
        await expect(page.locator('text=Standalone Network')).toBeVisible();
        await expect(page.locator('text=High-Concurrency')).toBeVisible();
        await expect(page.locator('text=Shared State Integrity')).toBeVisible();
    });

    test('should close when the Close button is clicked', async ({ page }) => {
        const chaosBtn = page.locator('button:has-text("Chaos Report")').first();
        await chaosBtn.click();

        const reportTitle = page.locator('text=Chaos Engineering & Parity Audit');
        await expect(reportTitle).toBeVisible();

        const closeBtn = page.locator('button:has-text("Close")').last();
        await closeBtn.click();

        await expect(reportTitle).not.toBeVisible();
    });
});
