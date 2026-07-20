import { test, expect } from '../../../../e2e/fixtures';

test.use({
  launchOptions: {
    args: [
      '--use-fake-ui-for-media-stream',
      '--use-fake-device-for-media-stream',
    ],
  },
});

test.describe('Voice Assistant Offline Sync', () => {
  test('should queue voice command when offline and display sync status', async ({ page, context }) => {
    await context.grantPermissions(['microphone']);

    // Navigate using UI interactions instead of direct local storage bypass
    await page.goto('/login');

    // Login natively
    await page.getByPlaceholder(/Email|Username/i).fill('test@onehumancorp.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In|Sign In/i }).click();

    // Fill onboarding if presented or skip
    try {
      const onboardButton = page.getByRole('button', { name: /Complete Onboarding|Next|Skip/i });
      if (await onboardButton.isVisible({ timeout: 3000 })) {
        await onboardButton.click();
      }
    } catch(e) {}

    await expect(page).toHaveURL(/.*\/dashboard/);

    await context.setOffline(true);

    const voiceButton = page.locator('button[aria-label="Voice Assistant"]').first();
    await expect(voiceButton).toBeVisible();

    // Start recording
    await voiceButton.dispatchEvent('mousedown');

    // Playwright natively simulating a mic thanks to use-fake-device-for-media-stream
    await expect(page.getByText(/Listening.../i)).toBeVisible();

    // Stop recording
    await voiceButton.dispatchEvent('mouseup');

    // Check processing state
    await expect(page.getByText(/Processing command.../i)).toBeVisible();

    // Due to offline mode, the system should queue the action
    await expect(page.getByText(/\(Queued for Sync\)/i)).toBeVisible({ timeout: 5000 });

    // Restore network to verify sync triggers
    await context.setOffline(false);

    // Give it time to sync and for the backend to process the task
    const newProposal = page.getByText(/Drafted Voice Order/i);
    await expect(newProposal).toBeVisible({ timeout: 15000 });
  });
});
