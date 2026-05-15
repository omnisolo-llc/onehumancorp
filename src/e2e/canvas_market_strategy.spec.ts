import { test, expect } from '@playwright/test';

test.describe('Canvas: OHC Market Strategy & UI Polish', () => {
    test.beforeEach(async ({ page }) => {
        // Start from login screen as mandated by requirements
        await page.goto('/');
        await page.waitForLoadState('networkidle');
    });

    test('Full CUJ 1: Login and verify Dashboard metrics and Website Preview widget', async ({ page }) => {
        await expect(page.locator('#login-screen')).toBeVisible({ timeout: 5000 });
        await page.fill('#login-email', 'owner@business.com');
        await page.fill('#login-password', 'password123');
        await page.click('button:has-text("Login")');

        // Verify Dashboard is visible
        await expect(page.locator('#dashboard-screen')).toBeVisible({ timeout: 5000 });
        await expect(page.locator('h1:has-text("Dashboard")')).toBeVisible();

        // Verify top metrics
        await expect(page.locator('h3:has-text("Today\'s Revenue")')).toBeVisible();
        await expect(page.locator('h3:has-text("New Orders")')).toBeVisible();

        // Verify Website Preview Widget
        await expect(page.locator('h2:has-text("Website Preview")')).toBeVisible();
        await expect(page.locator('.preview-widget')).toBeVisible();
    });

    test('Full CUJ 2: Navigate to Business Manager and add a new product', async ({ page }) => {
        // Login flow
        await page.fill('#login-email', 'owner@business.com');
        await page.fill('#login-password', 'password123');
        await page.click('button:has-text("Login")');
        await expect(page.locator('#dashboard-screen')).toBeVisible({ timeout: 5000 });

        // Navigate to Business
        await page.click('#nav-business');
        await expect(page.locator('#business-manager-screen')).toBeVisible({ timeout: 5000 });

        // Open add product modal
        await page.click('button:has-text("+ Add Product")');
        await expect(page.locator('#add-product-modal')).toBeVisible({ timeout: 5000 });

        // Fill product details
        await page.fill('#new-product-name', 'Artisan Coffee');
        await page.fill('#new-product-price', '15.00');

        // Handle alert
        page.once('dialog', dialog => dialog.accept());
        await page.click('button:has-text("Save Product")');
    });

    test('Full CUJ 3: Navigate to Inbox and use AI suggested reply', async ({ page }) => {
        // Login flow
        await page.fill('#login-email', 'owner@business.com');
        await page.fill('#login-password', 'password123');
        await page.click('button:has-text("Login")');
        await expect(page.locator('#dashboard-screen')).toBeVisible({ timeout: 5000 });

        // Navigate to Inbox
        await page.click('#nav-inbox');
        await expect(page.locator('#inbox-screen')).toBeVisible({ timeout: 5000 });

        // Click AI suggested reply
        await page.click('button:has-text("Yes, we have a wonderful")');

        // Check input value
        const inputValue = await page.inputValue('#reply-input');
        expect(inputValue).toContain('Yes, we have a wonderful');

        // Handle alert
        page.once('dialog', dialog => dialog.accept());
        await page.click('button:has-text("Send")');
    });

    test('Full CUJ 4: Navigate to Analytics and verify charts', async ({ page }) => {
        // Login flow
        await page.fill('#login-email', 'owner@business.com');
        await page.fill('#login-password', 'password123');
        await page.click('button:has-text("Login")');
        await expect(page.locator('#dashboard-screen')).toBeVisible({ timeout: 5000 });

        // Navigate to Analytics
        await page.click('#nav-analytics');
        await expect(page.locator('#analytics-screen')).toBeVisible({ timeout: 5000 });

        // Verify charts are visible
        await expect(page.locator('h2:has-text("Revenue (Last 7 Days)")')).toBeVisible();
        await expect(page.locator('.bar-chart')).toBeVisible();
        await expect(page.locator('.bar').first()).toBeVisible();
    });

    test('Full CUJ 5: Navigate to Agents and verify Swarm Activity', async ({ page }) => {
        // Login flow
        await page.fill('#login-email', 'owner@business.com');
        await page.fill('#login-password', 'password123');
        await page.click('button:has-text("Login")');
        await expect(page.locator('#dashboard-screen')).toBeVisible({ timeout: 5000 });

        // Verify Swarm Activity on Dashboard
        await expect(page.locator('h2:has-text("Swarm Activity")')).toBeVisible();
        await expect(page.locator('.feed-item').first()).toBeVisible();

        // Navigate to Agents
        await page.click('#nav-agents');
        await expect(page.locator('#agents-screen')).toBeVisible({ timeout: 5000 });

        // Verify Agents list
        await expect(page.locator('h2:has-text("Support Agent")')).toBeVisible();
        await expect(page.locator('h2:has-text("Order Manager")')).toBeVisible();
        await expect(page.locator('h2:has-text("Marketing Co-pilot")')).toBeVisible();
    });
});
