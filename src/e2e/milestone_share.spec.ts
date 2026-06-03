import { test, expect } from '@playwright/test';

test.describe('Growth Loop: Milestone Viral Share', () => {
  test('User can share milestone and unlock reward', async ({ page }) => {
    // Reset local storage to ensure the banner is not dismissed
    await page.goto('/');
    await page.evaluate(() => localStorage.removeItem('milestone_banner_dismissed'));

    // Navigate to the dashboard
    await page.goto('/dashboard');

    // Wait for the Milestone Growth Loop component to appear
    await expect(page.locator('text=Milestone Unlocked').first()).toBeVisible({ timeout: 15000 });

    // Verify the share button is visible
    const shareBtn = page.locator('text=Share & Claim Reward');
    await expect(shareBtn.first()).toBeVisible();

    // Handle any window dialogs (e.g., window.alert for success message)
    page.on('dialog', async dialog => {
      expect(dialog.message()).toContain('Awesome! Your 7-day Pro Trial Extension has been unlocked.');
      await dialog.accept();
    });

    // Create a mock for window.open to prevent new tabs from opening and failing the test unexpectedly
    await page.addInitScript(() => {
        (window as any).open = function(url: string, target: string) {
            console.log('Intercepted window.open:', url);
            return null;
        };
    });

    // Click the share button
    await shareBtn.first().click();

    // Verify the reward text updates on the frontend

  });
});
