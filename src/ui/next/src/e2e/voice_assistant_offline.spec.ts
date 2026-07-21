import { test, expect } from '../../../../e2e/fixtures';

test.describe('Voice Assistant Offline Sync', () => {
  test('should queue voice command when offline and display sync status', async ({ page, context }) => {
    await context.grantPermissions(['microphone']);
await page.goto('/dashboard');

    // Complete onboarding via UI to set has_onboarded instead of mutating localStorage directly
    // Assuming /onboarding sets this state. If the app redirects to onboarding, complete it.
    // Or, start from the proper state. For now, since the dashboard is accessible, we just go there.


    // Set offline mode using Playwright's native context method
    await context.setOffline(true);

    const voiceButton = page.locator('button[aria-label="Voice Assistant"]').first();
    await expect(voiceButton).toBeVisible();

    // Start recording
    await voiceButton.dispatchEvent('mousedown');
    await expect(page.getByText(/Listening.../i)).toBeVisible();

    // Stop recording
    await voiceButton.dispatchEvent('mouseup');

    // Check processing state
    await expect(page.getByText(/Processing command.../i)).toBeVisible();

    // Should indicate it was queued for sync
    await expect(page.getByText(/\(Queued for Sync\)/i)).toBeVisible({ timeout: 5000 });

    // Restore network to verify sync
    await context.setOffline(false);

    // Give it time to sync and for the backend to process the task,
    // which should result in an approval card in the feed.
    const newProposal = page.getByText(/Drafted Voice Order/i);
    await expect(newProposal).toBeVisible({ timeout: 15000 });
  });
});
