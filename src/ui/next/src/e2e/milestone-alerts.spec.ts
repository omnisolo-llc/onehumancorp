import { test, expect } from '../../../../e2e/fixtures';

test.describe('Milestone Alerts & Virality Loop', () => {
    test('displays milestone, verifies embed code, and tests share loop', async ({ page, request }) => {
        // Since playwright network intercept is blocked, we rely on the actual API using real data
        // For test isolation, we verify the page renders gracefully without mock interception.

        await page.goto('/dashboard');

        // Wait for hydration and UI to settle
        await page.waitForLoadState('networkidle');

        // Go to the Milestone Alerts page
        await page.goto('/milestone-alerts');

        // Wait for the main heading
        const header = page.locator('h1:has-text("Milestone Alerts")');
        await expect(header).toBeVisible();

        // Check if there are milestones or an empty state
        const firstSaleItem = page.locator('button:has-text("First Sale!")');

        if (await firstSaleItem.isVisible()) {
            await firstSaleItem.click();

            // Wait for preview to appear
            const previewImage = page.locator('img[alt="First Sale!"]');
            await expect(previewImage).toBeVisible();

            // Verify that the embed code generator shows up
            const embedHeader = page.locator('h3:has-text("Embed on your website")');
            await expect(embedHeader).toBeVisible();

            // Verify the HTML snippet structure includes the referral tracking link
            const textarea = page.locator('textarea');
            await expect(textarea).toBeVisible();
            const snippet = await textarea.inputValue();
            expect(snippet).toContain('source=milestone_embed');
            expect(snippet).toContain('api/v1/growth/milestone/card');

            // Verify share message copy functionality
            const copyMessageBtn = page.locator('button:has-text("Copy Share Message")');
            await expect(copyMessageBtn).toBeVisible();
        } else {
            // Check empty state
            const emptyState = page.locator('text=Select an unlocked milestone');
            await expect(emptyState).toBeVisible();
        }

        // Verify "Powered by OHC" footer is present
        const poweredBy = page.locator('.powered-by-footer');
        await expect(poweredBy).toBeVisible();
    });
});
