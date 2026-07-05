import { test, expect } from './fixtures';

test.describe('Omnichannel Voice Order Intake', () => {
  test.use({
    viewport: { width: 375, height: 812 }, // Mobile-first constraint
    permissions: ['microphone'],           // Allow microphone access to prevent NotAllowedError
    launchOptions: {
      args: [
        '--use-fake-ui-for-media-stream',
        '--use-fake-device-for-media-stream'
      ]
    }
  });

  test('Fatima can initiate a voice order from the dashboard', async ({ page }) => {
    // 1. Fatima opens the app and starts on the dashboard
    await page.goto('/dashboard');

    // 2. She sees the Voice Assistant button
    const voiceBtn = page.getByLabel('Voice Assistant');
    await expect(voiceBtn).toBeVisible();

    // 3. She holds the button to start recording the order
    await voiceBtn.dispatchEvent('mousedown');

    // 4. Verify UI reflects the recording state
    await expect(page.getByText('Listening...')).toBeVisible();

    // 5. She releases the button
    await voiceBtn.dispatchEvent('mouseup');

    // 6. Verify processing state
    await expect(page.getByText('Processing command...')).toBeVisible();

    // 7. Success state verifies the processing is complete
    await expect(page.getByText('Action Prepared!')).toBeVisible({ timeout: 5000 });

    // 8. Find the created draft intent in the Agent Feed/Inbox
    const feed = page.locator('section', { hasText: 'Unified Agent Feed' });
    await expect(feed).toBeVisible();

    // 9. Confirm the order intent was created (assert loosely to account for LLM variation)
    // The structured item 'Chicken Rice' should appear from the extracted JSON intent payload
    const orderCard = page.getByText('Chicken Rice');
    await expect(orderCard).toBeVisible();

    // 10. Approve the drafted order
    const approveBtn = page.getByTestId('feed-approve-btn').first();
    await approveBtn.waitFor({ state: 'visible', timeout: 5000 });
    await approveBtn.click();
  });
});
