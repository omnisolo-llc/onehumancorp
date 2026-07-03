import { test, expect } from './fixtures';

test.describe('Unboxing Share Generator E2E', () => {
    test('should allow member to customize unboxing share card and generate campaign', async ({ memberPage }) => {
        // Navigate to the unboxing share generator via the dashboard link
        await memberPage.goto('/dashboard.html');
        const link = memberPage.locator('#unboxing-share-link');
        await expect(link).toBeVisible();
        await link.click();

        await memberPage.waitForURL('**/unboxing-share-generator.html');

        await expect(memberPage.locator('h1.font-outfit')).toHaveText('Unboxing Share Generator');

        // Fill out the form
        await memberPage.fill('#productName', 'Test Product');
        await memberPage.fill('#hashtag', '#TestTag');
        await memberPage.fill('#reward', 'Test Reward');
        await memberPage.selectOption('#theme', 'dark');

        // Check the live preview updates
        await expect(memberPage.locator('#preview-brand')).toHaveText('Test Product');
        await expect(memberPage.locator('#preview-hashtag')).toHaveText('#TestTag');
        await expect(memberPage.locator('#preview-reward')).toHaveText('Test Reward');
        await expect(memberPage.locator('#preview-card')).toHaveClass(/dark-theme/);

        // Click generate
        const generateBtn = memberPage.locator('[data-testid="generate-btn"]');
        await generateBtn.click();

        // Check for success message
        await expect(memberPage.locator('#status-msg')).toBeVisible({ timeout: 5000 });
        await expect(memberPage.locator('#status-msg')).toHaveText('Campaign Activated!');
    });
});
