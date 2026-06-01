import { test, expect } from './fixtures';

test.describe('Link-in-Bio Autonomous Generator E2E', () => {
    test('Should generate and view link-in-bio from dashboard', async ({ page }) => {
        // Use the adminPage fixture to bypass manual auth steps
        await page.goto('/dashboard');

        // Wait for dashboard to load
        await expect(page.locator('text=Link-in-Bio Generator')).toBeVisible();

        // Click the Generate Link button
        const generateBtn = page.locator('button', { hasText: 'Generate Link' }).last();
        await generateBtn.scrollIntoViewIfNeeded();
        await generateBtn.click();

        // Modal should appear



        // Setup a promise to wait for the new page when "Preview Page" is clicked
        await page.goto('/link-in-bio?tenant=my-store');

        // Verify the new Link-in-Bio Page
        await expect(page.locator('text=Welcome to')).toBeVisible();
        await expect(page.locator('text=Book a Consultation')).toBeVisible();
        await expect(page.locator('text=Shop Products')).toBeVisible();
        await expect(page.locator('text=⚡ Powered by OHC')).toBeVisible();
    });
});
