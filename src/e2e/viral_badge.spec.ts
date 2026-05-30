import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Viral Badge E2E', () => {
    test('User can configure and copy the viral badge embed code', async ({ page }) => {
        // Use the adminPage fixture which signs in the test user globally and injects the storage state
        await adminPage({ page }, async ({ page }) => {
            // 1. Navigate to dashboard
            await page.goto('/dashboard');

            // 2. Click the link to the viral badge page
            const viralBadgeBtn = page.getByRole('button', { name: 'Configure Viral Badge' });
            await expect(viralBadgeBtn).toBeVisible();
            await viralBadgeBtn.click();

            // 3. Ensure we are on the viral badge page
            await expect(page).toHaveURL(/.*\/viral-badge/);
            await expect(page.getByRole('heading', { name: 'Viral Badge 🚀' })).toBeVisible();

            // 4. Test theme customization
            const darkThemeBtn = page.getByRole('button', { name: 'Dark' });
            await darkThemeBtn.click();

            // Check that the embed code updates based on the theme
            const embedCodePre = page.locator('pre');
            await expect(embedCodePre).toContainText('background: #1D1D1F');

            // 5. Test position customization
            const bottomLeftBtn = page.getByRole('button', { name: 'Bottom Left' });
            await bottomLeftBtn.click();
            await expect(embedCodePre).toContainText('bottom: 24px; left: 24px;');

            // 6. Test copy to clipboard
            // Note: Playwright needs permission to read/write clipboard. Instead of real copy, we click the button and check the visual change.
            const copyBtn = page.getByRole('button', { name: 'Copy Embed HTML' });
            await copyBtn.click();
            await expect(copyBtn).toHaveText('Copied to Clipboard!');
        });
    });
});
