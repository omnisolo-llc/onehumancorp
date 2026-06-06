import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Promoter Agent E2E', () => {
    test('surfaces 3 variant captions for a new product and allows scheduling', async ({ page }) => {
        // We use real data seeded by `e2e-seed.sql` which includes 'e2e-approval-promoter'.

        // Navigate to Dashboard (where UnifiedAgentFeed is rendered)
        await adminPage(page);
        await page.goto('/dashboard');

        // Verify the agent feed shows the card
        await expect(page.locator('text=New product detected! Schedule a post to drive sales?')).toBeVisible();
        await expect(page.locator('text=The Promoter has generated content for:')).toBeVisible();
        await expect(page.locator('text=Vegan Celebration Cake').first()).toBeVisible();

        // Verify the 3 variants exist
        await expect(page.locator('text=TikTok')).toBeVisible();
        await expect(page.locator('text=Get the new Vegan Celebration Cake! 🔥')).toBeVisible();

        await expect(page.locator('text=Instagram')).toBeVisible();
        await expect(page.locator('text=Check out our stunning new Vegan Celebration Cake ✨ #newarrival #shopping')).toBeVisible();

        await expect(page.locator('text=Facebook')).toBeVisible();
        await expect(page.locator('text=We are excited to announce our new product: Vegan Celebration Cake. Available now!')).toBeVisible();

        // Click "Approve" (Schedule)
        const postCard = page.locator('.bg-pink-50').locator('..').locator('..'); // Find the wrapper element for this approval card
        await postCard.locator('button:has-text("Approve")').click();

        // Verify Optimistic UI removed the card
        await expect(page.locator('text=New product detected! Schedule a post to drive sales?')).not.toBeVisible();
    });
});
