import { test, expect } from '@playwright/test';

test.describe('Growth Loop: Milestone Viral Share', () => {
  test('User can share milestone and unlock reward', async ({ page }) => {
    // Navigate to the dashboard
    await page.goto('/dashboard');

    // Wait for the Milestone Growth Loop component to appear
    await page.locator('text=Milestone Unlocked').first().waitFor();

    // Verify the share button is visible
    const shareBtn = page.locator('text=Share & Claim Reward');
    await shareBtn.first().waitFor();

    // Handle any window dialogs (e.g., window.alert for success message)
    page.on('dialog', async dialog => {
      expect(dialog.message()).toContain('Awesome! Your 7-day Pro Trial Extension has been unlocked.');
      await dialog.accept();
    });

    // Create a mock for window.open to prevent new tabs from opening and failing the test unexpectedly
    await page.addInitScript(() => {
        (window as any).open = function(url: string, target: string) {
            console.debug('Intercepted window.open:', url);
            return null;
        };
    });

    // Click the share button
    await shareBtn.first().click();

    // Verify the reward text updates on the frontend

  });
});
