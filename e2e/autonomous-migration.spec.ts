import { test, expect } from '@playwright/test';

test.describe('Autonomous Competitor Migration', () => {
    test('Maya migrates her Shopify store to OHC', async ({ page }) => {
        // Business Persona: Maya (Home Baker)
        // She wants to import her 42 cakes from her old Shopify store via URL.

        // 1. Navigate to dashboard/onboarding
        await page.goto('/dashboard');

        // Ensure logged in (assuming auto-login via fixture or session)

        // 2. Locate migration section in onboarding
        await page.click('text=Migrate Existing Store');

        // 3. Enter URL
        await page.fill('input[name="migration_url"]', 'mayas-cakes.myshopify.com');

        // 4. Submit
        await page.click('button:has-text("Start Migration")');

        // 5. Assert loading state (Glassmorphism loader)
        await expect(page.locator('text=Our AI is carefully moving your')).toBeVisible();

        // 6. Wait for migration to complete
        // In E2E, we might mock this or use a test fixture where the backend quickly resolves the job.
        await expect(page.locator('text=Migration Complete')).toBeVisible({ timeout: 15000 });

        // 7. Review imported products
        await page.click('button:has-text("Review & Publish")');

        // Should navigate to products catalog
        await expect(page).toHaveURL(/.*\/products/);

        // And we should see at least one imported cake in the list (mocked by Minimax locally or by the E2E seed script)
        await expect(page.getByText('Chocolate Cake')).toBeVisible();
    });
});
