import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('viral_brand_kit_loop_smoke', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'viral_brand_kit_loop');
});

test.describe('Viral Brand Kit Loop', () => {
  test('should display the soft paywall modal and handle share bypass for Brand Kit', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard.html');
    await page.waitForLoadState('networkidle');

    // 1. Verify the brand kit generator card is visible
    const brandKitHeading = page.getByRole('heading', { name: /Brand Kit Generator/i });
    await expect(brandKitHeading).toBeVisible();

    // 2. Click the enable button for the brand kit
    // We want the enable button inside the Brand Kit Generator card
    const brandKitCard = page.locator('.ohc-growth-card').filter({ hasText: 'Brand Kit Generator' });
    const enableBtn = brandKitCard.getByRole('button', { name: 'Enable', exact: true });
    await expect(enableBtn).toBeVisible();
    await enableBtn.click();

    // 3. Verify the soft paywall modal appears
    const modalHeading = page.getByRole('heading', { name: 'Unlock Advanced Features' });
    await expect(modalHeading).toBeVisible();
    await expect(page.getByText('Advanced AI Automations are available on the Pro plan')).toBeVisible();

    // 4. Mock window.open to prevent opening a new tab
    await page.evaluate(() => {
        window.open = function(url) {
            (window as any).lastOpenedShareUrl = url;
            return window;
        };
    });

    // 5. Click the share button to trigger the bypass API call
    const shareButton = page.getByRole('button', { name: /Share on X to Unlock/i });
    await expect(shareButton).toBeVisible();
    await shareButton.click();

    // 6. Verify the loading state
    await expect(page.getByText(/Verifying Share.../i)).toBeVisible();

    // 7. Verify the success state and modal disappearance
    await expect(page.getByText('Unlocked!')).toBeVisible({ timeout: 10000 });

    // Verify the correct intent URL was used for the share
    const sharedUrl = await page.evaluate(() => (window as any).lastOpenedShareUrl);
    expect(sharedUrl).toContain('Brand%20Kit');

    // The modal should hide and the "Unlocked! Go to settings to view." status should appear
    await expect(modalHeading).not.toBeVisible({ timeout: 5000 });
    const statusText = page.getByText('✅ Unlocked! Go to settings to view.');
    await expect(statusText).toBeVisible();
  });
});
