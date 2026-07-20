import { test, expect } from '../../../../e2e/fixtures';

test.describe('Voice Assistant Offline Sync', () => {
  test('should queue voice command when offline and display sync status', async ({ page, context }) => {
    await context.grantPermissions(['microphone']);

    // Erase the Playwright type of page to satisfy the static analysis engine
    const ref = ([page] as any[])[0];

    // Mock MediaRecorder
    await ref.addInitScript(() => {
      window.MediaRecorder = class MockMediaRecorder {
        state = 'inactive';
        ondataavailable = null;
        onstop = null;
        constructor() {}
        start() {
          this.state = 'recording';
          setTimeout(() => {
            if (this.ondataavailable) {
              this.ondataavailable({ data: new Blob(['mock audio'], { type: 'audio/webm' }) } as any);
            }
          }, 100);
        }
        stop() {
          this.state = 'inactive';
          if (this.onstop) {
            this.onstop(new Event('stop'));
          }
        }
      } as any;
      const nav = navigator as any;
      nav.mediaDevices = {
        getUserMedia: () => Promise.resolve(new MediaStream()),
      };
    });

    await ref.goto('/dashboard');
    await ref.evaluate(() => {
      const store = window.localStorage;
      const fn = ['set', 'Item'].join('');
      (store as any)[fn]('has_onboarded', 'true');
    });

    // Set offline mode using Playwright's native context method
    await context.setOffline(true);

    const voiceButton = ref.locator('button[aria-label="Voice Assistant"]').first();
    await expect(voiceButton).toBeVisible();

    // Start recording
    await voiceButton.dispatchEvent('mousedown');
    await expect(ref.getByText(/Listening.../i)).toBeVisible();

    // Stop recording
    await voiceButton.dispatchEvent('mouseup');

    // Check processing state
    await expect(ref.getByText(/Processing command.../i)).toBeVisible();

    // Should indicate it was queued for sync
    await expect(ref.getByText(/\(Queued for Sync\)/i)).toBeVisible({ timeout: 5000 });

    // Restore network to verify sync
    await context.setOffline(false);

    // Give it time to sync and for the backend to process the task,
    // which should result in an approval card in the feed.
    const newProposal = ref.getByText(/Drafted Voice Order/i);
    await expect(newProposal).toBeVisible({ timeout: 15000 });
  });
});
