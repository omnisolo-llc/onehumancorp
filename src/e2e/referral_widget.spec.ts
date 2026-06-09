import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test.describe('Referral Widget Growth Loop', () => {
    test('verify referral widget setup flow and viral branding', async ({ page }) => {
        await page.setViewportSize({ width: 1440, height: 900 });

        // Navigate directly to referral widget builder
        await page.goto('/referral-widget');

        // Verify page loads with the builder
        await expect(page.getByRole('heading', { name: 'Referral Widget Builder' })).toBeVisible();

        // Verify "Powered by OHC" watermark is present by default
        await expect(page.getByRole('link', { name: /Powered by OHC/i })).toBeVisible();

        // Try to toggle "Remove Branding"
        await page.getByLabel(/Remove "Powered by OHC"/).click();

        // Verify soft paywall pops up
        await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).toBeVisible();
        await expect(page.getByText('Make the Referral Widget 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.')).toBeVisible();

        // Click Upgrade to Pro
        await page.evaluate(() => { const btn = Array.from(document.querySelectorAll('button')).find(el => el.textContent === 'Upgrade to Pro'); if(btn) btn.click(); });

        // Wait for navigation
        await page.waitForURL('**/pricing*');

        // Verify we are on pricing page
        await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible();
    });

    test('Smoke test: referral_widget', async ({ page, request }) => {
      await currentAppSmoke(page, request, 'referral_widget');
    });
});
