import { test, expect } from './fixtures';

test.describe('Voice Assistant Command Center', () => {
  test('Carlos can initiate a quote via voice command hands-free', async ({ page }) => {
    // 1. Start from the dashboard (Carlos's home screen)
    await page.goto('/dashboard');

    // 2. Locate the floating Voice Assistant button (glassmorphism style)
    const voiceBtn = page.getByLabel('Voice Assistant');
    await expect(voiceBtn).toBeVisible();

    // 3. Carlos holds to speak (simulate push-to-talk)
    // We simulate the interaction. In a real device, this would use touch events.
    await voiceBtn.dispatchEvent('mousedown');

    // 4. Verify "Listening" state and waveform animation
    await expect(page.getByText('Listening...')).toBeVisible();

    // 5. Release to process command
    await voiceBtn.dispatchEvent('mouseup');

    // 6. Verify "Thinking" state
    await expect(page.getByText('Thinking...')).toBeVisible();

    // 7. Success state and transcription display
    // Our mock backend returns a fixed transcription for the demo
    await expect(page.getByText('Action Prepared!')).toBeVisible({ timeout: 5000 });
    await expect(page.getByText(/Create a \$150 repair quote/)).toBeVisible();

    // 8. Verify the proposed action card appears in the Agent Feed
    // Navigate to Triage/Feed if not on the same page, but UnifiedAgentFeed is on Dashboard
    const feed = page.locator('section', { hasText: 'Unified Agent Feed' });
    await expect(feed).toBeVisible();

    // Check for the new proposal card
    const actionCard = page.getByTestId('quote-draft-card').first();
    await expect(actionCard).toBeVisible();
    await expect(actionCard).toContainText('$150');

    // 9. Carlos can approve the quote with one tap
    const approveBtn = page.getByTestId('feed-approve-btn').first();
    await approveBtn.click();

    // 10. Verify card is cleared or marked as approved in activity
    await expect(page.getByText('Approved!')).toBeVisible();
  });

  test('Voice Assistant button follows glassmorphism design tokens', async ({ page }) => {
    await page.goto('/dashboard');
    const voiceBtn = page.getByLabel('Voice Assistant');

    // Verify mobile-first sizing (roughly 64x64 as implemented)
    const box = await voiceBtn.boundingBox();
    expect(box?.width).toBe(64);
    expect(box?.height).toBe(64);

    // Check for glassmorphism styles (backdrop-filter)
    const computedStyle = await voiceBtn.evaluate((el) => {
        return window.getComputedStyle(el).backdropFilter || window.getComputedStyle(el).webkitBackdropFilter;
    });
    expect(computedStyle).toContain('blur');
  });
});
