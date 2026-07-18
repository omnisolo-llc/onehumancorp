import { test, expect } from './fixtures';

test.use({
  viewport: { width: 375, height: 812 },
  permissions: ['microphone'],
  launchOptions: {
    args: [
      '--use-fake-ui-for-media-stream',
      '--use-fake-device-for-media-stream'
    ]
  }
});

test.describe('Omnichannel Voice Order Intake', () => {
  test.setTimeout(120000);

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
    await expect(page.getByText('Action Prepared!')).toBeVisible({ timeout: 15000 });

    // 8. Confirm the order intent was translated to the result card correctly
    // The structured item 'Chicken Tacos' should appear from the extracted JSON intent payload
    // based on our mock translation string "Quiero 3 tacos de pollo" -> "3x Chicken Tacos"
    const orderCard = page.getByText('Chicken Tacos');
    await expect(orderCard).toBeVisible();

    // 9. Confirm the drafted order via the walk-up UI
    const confirmBtn = page.getByRole('button', { name: 'Confirm & Add to List' });
    await expect(confirmBtn).toBeVisible();
    await confirmBtn.click();

    // 10. Find the created draft intent in the Agent Feed/Inbox (after modal closes)
    const feed = page.locator('section', { hasText: 'Unified Agent Feed' });
    await expect(feed).toBeVisible();

    // The item should now be in the feed
    await expect(page.getByText('Chicken Tacos').first()).toBeVisible();

    // 11. Approve the drafted order in the feed
    const approveBtnFeed = page.getByTestId('feed-approve-btn').first();
    await approveBtnFeed.waitFor({ state: 'visible', timeout: 5000 });
    await approveBtnFeed.click();
  });
});
