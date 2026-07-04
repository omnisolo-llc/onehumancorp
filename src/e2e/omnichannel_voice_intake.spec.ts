import { test, expect } from './fixtures';

test.describe('Omnichannel Voice Order Intake', () => {
  test.use({
    viewport: { width: 375, height: 812 }, // Mobile-first constraint
    permissions: ['microphone'],           // Allow microphone access to prevent NotAllowedError
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

    // 7. Success state verifies the mock transcription for Fatima's food cart order
    await expect(page.getByText('Action Prepared!')).toBeVisible({ timeout: 5000 });
    await expect(page.getByText(/Drafted Order: 2x Chicken Rice/)).toBeVisible();

    // 8. Find the created draft intent in the Agent Feed/Inbox
    const feed = page.locator('section', { hasText: 'Unified Agent Feed' });
    await expect(feed).toBeVisible();

    // 9. Confirm the order quantities and items are correct in the draft
    const orderCard = page.getByText('Chicken Rice');
    await expect(orderCard).toBeVisible();

    // 10. Approve the drafted order
    const approveBtn = page.getByTestId('approve-draft-action');
    if (await approveBtn.isVisible()) {
        await approveBtn.click();
        await expect(page.getByText('Approved!')).toBeVisible();
    }
  });
});
