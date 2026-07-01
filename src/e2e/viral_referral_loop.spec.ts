import { test, expect } from '@playwright/test';
import { loginAs } from './fixtures';

test.describe('Viral Referral Loop', () => {
    test.use({ viewport: { width: 375, height: 667 } }); // Mobile first

    test('should display "Give $50, Get $50" on the referral dashboard', async ({ page }) => {
        // Authenticate
        const user = await loginAs(page, 'test@example.com', 'password123');

        // Navigate to dashboard
        await page.goto('/dashboard.html');

        // Click on Referral Dashboard link
        const referralLink = page.locator('#generate-link-btn');
        await expect(referralLink).toBeVisible();
        await referralLink.click();

        // Verify URL
        await expect(page).toHaveURL(/.*referrals\.html/);

        // Verify the text content is updated
        await expect(page.locator('p', { hasText: 'Give $50, Get $50' })).toBeVisible();

        // Verify metric blocks
        await expect(page.locator('#metrics-invites')).toBeVisible();
        await expect(page.locator('#metrics-active')).toBeVisible();
        await expect(page.locator('#metrics-revenue')).toBeVisible();
        await expect(page.locator('#metrics-pending')).toBeVisible();

        // Verify share native button exists
        await expect(page.locator('#share-native-btn')).toBeVisible();
    });
});
