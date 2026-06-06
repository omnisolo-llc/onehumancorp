import { test, expect } from './fixtures';

test.describe('Growth Loop: Milestone Viral Share', () => {
  test('User can share milestone and unlock reward', async ({ page }) => {
    // Navigate to the dashboard
    await page.goto('/dashboard');

    // Wait for the Milestone Growth Loop component to appear
    await page.locator('text=Milestone Unlocked').first().waitFor();

    // Verify the share button is visible
    const shareBtn = page.locator('text=Share & Claim Reward');
    await shareBtn.first().waitFor();

    // Create a mock for window.open to prevent new tabs from opening and failing the test unexpectedly
    await page.addInitScript(() => {
        (window as any).open = function(url: string, target: string) {
            console.debug('Intercepted window.open:', url);
            return null;
        };
    });

    // Also handle navigator.clipboard mock since playwright tests may not have full clipboard permissions
    await page.addInitScript(() => {
        Object.assign(navigator, {
            clipboard: {
                writeText: () => Promise.resolve(),
            },
        });
    });

    // Click the share button
    await shareBtn.first().click();

    // Verify the reward text updates on the frontend
    await expect(page.locator('text=Reward claimed. Invite link copied.')).toBeVisible();
  });
});
