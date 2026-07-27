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
        await expect(page.getByText(/Make the Referral Widget 100% yours/)).toBeVisible();

        // Setup tested window.open so the share button doesn't actually open a new tab and break tests
        await page.evaluate(() => {
            window.open = function() { return null; };
        });

        // Click Share to Unlock
        const shareBtn = page.getByRole('button', { name: /Share on X to Unlock 7 Days/i });
        await expect(shareBtn).toBeVisible();
        await shareBtn.click();

        // Verify loading state
        await expect(page.getByText('Verifying Share...')).toBeVisible();

        // Verify success state
        await expect(page.getByText('Unlocked!')).toBeVisible({ timeout: 10000 });

        // Verify modal closes and checkbox is now checked
        await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).toBeHidden({ timeout: 5000 });
        const checkbox = page.getByLabel(/Remove "Powered by OHC"/);
        await expect(checkbox).toBeChecked();

        // Verify branding is removed from preview
        await expect(page.getByRole('link', { name: /Powered by OHC/i })).toBeHidden();
    });

    test('Smoke test: referral_widget', async ({ page, request }) => {
      await currentAppSmoke(page, request, 'referral_widget');
    });
});
