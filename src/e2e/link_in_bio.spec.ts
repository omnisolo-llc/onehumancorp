import { test, expect } from './fixtures';

test.describe('Link-in-Bio Autonomous Generator E2E', () => {
    test('Should generate and view link-in-bio from dashboard', async ({ page }) => {
        // Use the adminPage fixture to bypass manual auth steps
        await page.goto('/dashboard');

        // Wait for dashboard to load
        await expect(page.locator('text=Link-in-Bio Generator')).toBeVisible();

        // Click the Generate Link button
        const generateBtn = page.locator('text=Generate Link');
        await generateBtn.click();

        // Modal should appear
        await expect(page.locator('text=Your Link-in-Bio is Ready')).toBeVisible();

        // Setup a promise to wait for the new page when "Preview Page" is clicked
        const [newPage] = await Promise.all([
            page.waitForEvent('popup'),
            page.locator('text=Preview Page').click()
        ]);

        await newPage.waitForLoadState('domcontentloaded');

        // Verify the new Link-in-Bio Page
        await expect(newPage.locator('text=Welcome to')).toBeVisible();
        await expect(newPage.locator('text=Book a Consultation')).toBeVisible();
        await expect(newPage.locator('text=Shop Products')).toBeVisible();
        await expect(newPage.locator('text=⚡ Powered by OHC')).toBeVisible();

        await newPage.close();
    });
});
