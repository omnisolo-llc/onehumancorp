import { test, expect } from '../../../../e2e/fixtures';

test.describe('Voice Assistant Offline Sync', () => {
  test('should queue voice command when offline and display sync status', async ({ page, context }) => {
    await context.grantPermissions(['microphone']);

    // Mock MediaRecorder
    await page.addInitScript(() => {
      class MockMediaRecorder {
        state = 'inactive';
        ondataavailable: ((event: BlobEvent) => void) | null = null;
        onstop: ((event: Event) => void) | null = null;
        constructor() {}
        start() {
          this.state = 'recording';
          setTimeout(() => {
            if (this.ondataavailable) {
              this.ondataavailable(new BlobEvent('dataavailable', { data: new Blob(['mock audio'], { type: 'audio/webm' }) }));
            }
          }, 100);
        }
        stop() {
          this.state = 'inactive';
          if (this.onstop) {
            this.onstop(new Event('stop'));
          }
        }
      }
      Object.defineProperty(window, 'MediaRecorder', { configurable: true, value: MockMediaRecorder });
      Object.defineProperty(navigator, 'mediaDevices', { configurable: true, value: {
        getUserMedia: () => Promise.resolve(new MediaStream()),
      } });
    });

    await page.goto('/dashboard');
    await page.evaluate(() => localStorage.setItem('has_onboarded', 'true'));

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
