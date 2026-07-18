import { test, expect } from '../../../../e2e/fixtures';

test.describe('Voice Assistant Mobile Command Center', () => {
  test('should process voice command and add Approval Card to Unified Agent Feed', async ({ page }) => {
    // 1. Setup mock permissions to allow microphone (we'll just stub the navigator API inside the test context if possible, but actually since we're using a simulated process, it might fail if microphone is blocked)
    // Actually, Playwright can grant permissions:
    await page.context().grantPermissions(['microphone']);

    // Mock the MediaRecorder API to avoid relying on actual microphone in CI
    await page.addInitScript(() => {
      window.MediaRecorder = class MockMediaRecorder {
        state = 'inactive';
        ondataavailable = null;
        onstop = null;

        constructor() {}

        start() {
          this.state = 'recording';
          // Simulate some data
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

      (navigator as any).mediaDevices = {
        getUserMedia: () => Promise.resolve(new MediaStream()),
      };
    });

    // 2. Go to dashboard
    await page.goto('/dashboard');
    await page.evaluate(() => localStorage.setItem('has_onboarded', 'true'));

    // 3. Find the Voice Assistant button
    const voiceButton = page.getByRole('button', { name: /Voice Command Assistant/i });
    await expect(voiceButton).toBeVisible();

    // 4. Simulate a long press to start recording
    // Since our button uses mousedown/mouseup, we dispatch those
    await voiceButton.dispatchEvent('mousedown');

    // Check that it indicates listening
    await expect(page.getByText(/Listening... Release to send/i)).toBeVisible();

    // 5. Release to stop recording and send command
    await voiceButton.dispatchEvent('mouseup');

    // Check processing state
    await expect(page.getByText(/Processing command.../i)).toBeVisible();

    // 6. Verify the proposed action appears in the Agent Feed (optimistic UI)
    // Looking for the title we mocked in the backend route
    const newProposal = page.getByText(/Voice Command: Send Quote/i);
    await expect(newProposal).toBeVisible({ timeout: 5000 });

    // 7. Verify the proposal is an actionable card (has Approve & Send Proposal button)
    const approveBtn = newProposal.locator('..').locator('..').getByTestId('feed-approve-btn');
    await expect(approveBtn).toBeVisible();
  });
});
