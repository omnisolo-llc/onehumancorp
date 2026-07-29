import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test.describe('Post-Purchase Share Widget Generator', () => {
    test('verify post-purchase widget setup flow and viral branding', async ({ page }) => {
        await page.setViewportSize({ width: 1440, height: 900 });

        // Navigate directly to post-purchase widget builder
        await page.goto('/post-purchase-share.html');

        // Verify page loads with the builder
        await expect(page.getByRole('heading', { name: 'Post-Purchase Share Widget' })).toBeVisible();

        // Verify "Powered by OHC" watermark is present by default in the live preview
        await expect(page.getByRole('link', { name: /Powered by OHC/i })).toBeVisible();

        // Try to toggle "Remove Branding"
        await page.getByLabel(/Remove "Powered by OHC"/).click();

        // Verify soft paywall pops up
        await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).toBeVisible();
        await expect(page.getByText(/Make the Post-Purchase Widget 100% yours/)).toBeVisible();

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

        // Verify embed API HTML renders successfully (integration with backend)
        const response = await page.request.get('/api/v1/growth/post-purchase/embed?tenant=test-tenant&discount=20pct&hideBranding=false');
        expect(response.ok()).toBeTruthy();
        const html = await response.text();

        // Check for presence of discount text and branding
        expect(html).toContain('Share and Get 20% OFF');
        expect(html).toContain('⚡ Powered by OHC');

        // Verify hideBranding parameter works on API
        const responseNoBranding = await page.request.get('/api/v1/growth/post-purchase/embed?tenant=test-tenant&discount=20pct&hideBranding=true');
        expect(responseNoBranding.ok()).toBeTruthy();
        const htmlNoBranding = await responseNoBranding.text();
        expect(htmlNoBranding).not.toContain('⚡ Powered by OHC');
    });

    test('Smoke test: post_purchase_share_widget', async ({ page, request }) => {
      await currentAppSmoke(page, request, 'post_purchase_share_widget');
    });
});
