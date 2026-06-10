import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test('smoke', async ({ page, request }) => {
  await test('smoke', async ({ page, request }) => {
  await currentAppSmoke(page, request, page, request, 'viral_soft_paywall_loop');
});
});

test.describe('Viral Soft Paywall Loop', () => {
  test('should display the soft paywall modal and handle share bypass', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    // 1. Verify the advanced automations card is visible
    const advancedAutomationsHeading = page.getByRole('heading', { name: /Advanced AI Automations/i });
    await expect(advancedAutomationsHeading).toBeVisible();

    // 2. Click the enable button
    const enableBtn = page.getByRole('button', { name: 'Enable', exact: true });
    await expect(enableBtn).toBeVisible();
    await enableBtn.click();

    // 3. Verify the soft paywall modal appears
    const modalHeading = page.getByRole('heading', { name: 'Unlock Advanced Features' });
    await expect(modalHeading).toBeVisible();
    await expect(page.getByText('Advanced AI Automations are available on the Pro plan')).toBeVisible();

    // 4. Mock window.open to prevent opening a new tab
    await page.evaluate(() => {
        window.open = function() { return window; };
    });

    // 5. Click the share button to trigger the bypass API call
    const shareButton = page.getByRole('button', { name: /Share on X to Unlock/i });
    await expect(shareButton).toBeVisible();
    await shareButton.click();

    // 6. Verify the loading state
    await expect(page.getByText(/Verifying Share.../i)).toBeVisible();

    // 7. Verify the success state and modal disappearance
    await expect(page.getByText('Unlocked!')).toBeVisible({ timeout: 10000 });

    // The modal should hide and the "Enabled" status should appear
    await expect(modalHeading).not.toBeVisible({ timeout: 5000 });
    await expect(page.getByText('✅ Enabled')).toBeVisible();
  });
});
