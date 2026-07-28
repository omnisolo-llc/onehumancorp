import { test, expect } from '@playwright/test';

test.describe('Growth Loop: Birthday Club Widget', () => {
    test('renders correctly with default params', async ({ page }) => {
        await page.goto('/api/v1/growth/birthday-club/embed');

        await expect(page.locator('text=Join Our Birthday Club')).toBeVisible();
        await expect(page.locator('text=15% OFF')).toBeVisible();
        await expect(page.locator('input[type="email"]')).toBeVisible();
        await expect(page.locator('input[type="date"]')).toBeVisible();

        const footerLink = page.locator('a:has-text("⚡ Powered by OHC")');
        await expect(footerLink).toBeVisible();
        await expect(footerLink).toHaveAttribute('href', /ref=demo/);
    });

    test('renders correctly with custom params', async ({ page }) => {
        await page.goto('/api/v1/growth/birthday-club/embed?tenant=mystore&discount=25');

        await expect(page.locator('text=25% OFF')).toBeVisible();

        const footerLink = page.locator('a:has-text("⚡ Powered by OHC")');
        await expect(footerLink).toBeVisible();
        await expect(footerLink).toHaveAttribute('href', /ref=mystore/);
    });

    test('hides branding when hideBranding is true', async ({ page }) => {
        await page.goto('/api/v1/growth/birthday-club/embed?hideBranding=true');

        await expect(page.locator('a:has-text("⚡ Powered by OHC")')).toBeHidden();
    });

    test('escapes XSS attempts', async ({ page }) => {
        await page.goto('/api/v1/growth/birthday-club/embed?discount=<script>alert("xss")</script>');

        // The script should be escaped and displayed as text, not executed
        await expect(page.locator('text=<script>alert("xss")</script>% OFF')).toBeVisible();
    });
});
