import { test, expect } from '@playwright/test';

test.describe('Link-in-Bio Generator Growth Loop', () => {
    test('dashboard shows the link-in-bio section', async ({ page }) => {
        // Go to dashboard
        await page.goto('http://localhost:3000/dashboard');

        // Look for the "Link-in-Bio Generator" section
        const sectionHeader = page.locator('h2:has-text("Link-in-Bio Generator")');
        await expect(sectionHeader).toBeVisible();

        // Check for the "Viral Growth" badge next to the header
        await expect(page.locator('text=Viral Growth').first()).toBeVisible();
    });

    test('opens the link-in-bio modal when clicking the button', async ({ page }) => {
        await page.goto('http://localhost:3000/dashboard');

        // Click "Get Link-in-Bio" button
        const getWidgetBtn = page.locator('button:has-text("Get Link-in-Bio")');
        await expect(getWidgetBtn).toBeVisible();
        await getWidgetBtn.click();

        // Modal should appear
        const modalHeader = page.locator('h2:has-text("Your Link-in-Bio")');
        await expect(modalHeader).toBeVisible();

        // Input should contain the URL
        const input = page.locator('input[value*="growth/link-in-bio"]');
        await expect(input).toBeVisible();
        const url = await input.inputValue();
        expect(url).toContain('/api/v1/growth/link-in-bio');
    });

    test('copy button functions correctly in the modal', async ({ page }) => {
        await page.goto('http://localhost:3000/dashboard');

        await page.locator('button:has-text("Get Link-in-Bio")').click();

        // Find copy button
        const copyBtn = page.locator('button:has-text("Copy")').filter({ hasText: /^Copy$/ });
        await expect(copyBtn).toBeVisible();

        await copyBtn.click();

        // Button text should change to Copied!
        await expect(page.locator('button:has-text("Copied!")')).toBeVisible();
    });

    test('preview link is present and correct in the modal', async ({ page }) => {
        await page.goto('http://localhost:3000/dashboard');

        await page.locator('button:has-text("Get Link-in-Bio")').click();

        // Find preview link
        const previewLink = page.locator('a:has-text("Preview Link")');
        await expect(previewLink).toBeVisible();

        const href = await previewLink.getAttribute('href');
        expect(href).toContain('/api/v1/growth/link-in-bio?tenant=');
    });

    test('API endpoint returns the correctly formatted HTML with growth loops', async ({ request }) => {
        const response = await request.get('http://localhost:3000/api/v1/growth/link-in-bio?tenant=maya-cakes');
        expect(response.ok()).toBeTruthy();

        const html = await response.text();

        // Check for correct tenant passing
        expect(html).toContain('maya-cakes');

        // Check for various links
        expect(html).toContain('Shop My Store');
        expect(html).toContain('Book a Session');

        // Ensure the referral growth loop is intact in the footer
        expect(html).toContain('Powered by');
        expect(html).toContain('OHC');
        expect(html).toContain('?ref=maya-cakes');
    });
});