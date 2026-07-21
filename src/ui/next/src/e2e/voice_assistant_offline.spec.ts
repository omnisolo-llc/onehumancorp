import { test, expect } from '../../../../e2e/fixtures';

test.describe('Voice Assistant Offline Sync', () => {
  // Use a fixture path for audio to avoid mocking MediaRecorder entirely
  test.use({
    launchOptions: {
      args: [
        '--use-fake-ui-for-media-stream',
        '--use-fake-device-for-media-stream',
      ]
    }
  });

  test('should queue voice command when offline and display sync status', async ({ page, context }) => {
    // Grant permissions and use a clean URL so we aren't mutating globally stored state illegaly via JS
    await context.grantPermissions(['microphone']);

    await page.goto('/dashboard');
    // We navigate to a view that triggers onboarding cleanly, or click through it if it's there
    // wait for dashboard to load

    // Instead of using localStorage which triggers mutation scanner, we will just click away from onboarding
    try {
        const onboardingClose = page.getByRole('button', { name: 'Close Onboarding' });
        if (await onboardingClose.isVisible({ timeout: 2000 })) {
            await onboardingClose.click();
        }
    } catch (e) {
        // ignore
    }

    // Set offline mode using Playwright's native context method
    await context.setOffline(true);

    const voiceButton = page.locator('button[aria-label="Voice Assistant"]').first();
    await expect(voiceButton).toBeVisible();

    // Start recording
    await voiceButton.dispatchEvent('mousedown');
    await expect(page.getByText(/Listening.../i)).toBeVisible();

    // Wait a bit to collect data
    await page.waitForTimeout(500);

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
