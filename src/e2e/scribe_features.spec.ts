import { test, expect } from '@playwright/test';

test.describe('Documentation Features Navigation E2E', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/login');
        await page.locator('input[type="email"]').fill('test@example.com');
        await page.locator('input[type="password"]').fill('password123');
        await page.locator('button:has-text("Sign In"), button:has-text("Login")').click();
        await page.waitForURL('**/dashboard**');
    });

    test('Help Center should open from main dashboard', async ({ page }) => {
        const helpButton = page.locator('text="?"').first();
        if (await helpButton.isVisible()) {
             await helpButton.click();
             await expect(page.locator('text="Help Center"')).toBeVisible();
             await expect(page.locator('text="Getting Started"')).toBeVisible();
             await expect(page.locator('text="My Store"')).toBeVisible();
        } else {
             await page.locator('button:has-text("Menu")').first().click();
             await page.locator('button:has-text("Help Center")').first().click();
             await expect(page.locator('text="Help Center"')).toBeVisible();
             await expect(page.locator('text="Getting Started"')).toBeVisible();
             await expect(page.locator('text="My Store"')).toBeVisible();
        }
    });

    test('AI Help Chat should open', async ({ page }) => {
        await page.locator('button:has-text("Menu")').first().click();
        await page.locator('button:has-text("AI Chat")').first().click();
        await expect(page.locator('text=/AI Help Assistant/i').first()).toBeVisible();
        const input = page.locator('input[placeholder*="question"]').first();
        await input.fill('How do I reset my password?');
    });

    test('Video Tutorials should open', async ({ page }) => {
        await page.locator('button:has-text("Menu")').first().click();
        await page.locator('button:has-text("Video Tutorials")').first().click();
        await expect(page.locator('text=/Watch & Learn/i').first()).toBeVisible();
    });

    test('App Tour should open', async ({ page }) => {
        await page.locator('button:has-text("Menu")').first().click();
        await page.locator('button:has-text("App Tour")').first().click();
        await expect(page.locator('text=/walkthrough|tour|guide/i')).toBeVisible();
    });

    test('API Documentation should open', async ({ page }) => {
        await page.locator('button:has-text("Menu")').first().click();
        await page.locator('button:has-text("Connect Apps")').first().click();
        await expect(page.locator('text=/Custom Integration/i').first()).toBeVisible();

        // Advanced mode should be hidden by default but togglable
        const advancedToggle = page.locator('text=/Advanced/i').first();
        if (await advancedToggle.isVisible()) {
            await advancedToggle.click();
            await expect(page.locator('text=/GET/i').first()).toBeVisible();
        }
    });

    test('Release Notes should open', async ({ page }) => {
        await page.locator('button:has-text("Menu")').first().click();
        await page.locator('button:has-text("What\'s New")').first().click();
        await expect(page.locator('text=/What\'s New/i').first()).toBeVisible();
        await expect(page.locator('text=/Version/i').first()).toBeVisible();
    });
});
