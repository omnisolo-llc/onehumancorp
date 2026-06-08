import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Work Intake Widget', () => {
    test('Should generate embed code and handle submission', async ({ page }) => {
        // We use the adminPage fixture to ensure we are logged in, but we'll manually navigate
        await adminPage.goto('/work-intake-widget');

        // Get the iframe src URL from the text area
        const embedCode = await adminPage.locator('textarea').first().inputValue();
        const srcMatch = embedCode.match(/src="([^"]+)"/);
        expect(srcMatch).not.toBeNull();
        const iframeSrc = srcMatch![1];

        // Navigate directly to the iframe source to test the widget functionality
        await page.goto(iframeSrc.replace('https://ohc.app', ''));

        // Verify the form is rendered
        await expect(page.locator('text=Work Request')).toBeVisible();
        await expect(page.locator('text=Name')).toBeVisible();
        await expect(page.locator('text=Email')).toBeVisible();

        // Fill and submit the form
        await page.fill('input[name="name"]', 'Playwright Test User');
        await page.fill('input[name="email"]', 'playwright@example.com');
        await page.fill('textarea[name="message"]', 'This is a test message from Playwright.');

        await page.click('button[type="submit"]');

        // Verify success message
        await expect(page.locator('text=Request Sent!')).toBeVisible();
        await expect(page.locator('text=We\'ll be in touch soon.')).toBeVisible();
    });
});
