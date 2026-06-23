import { test, expect } from '@playwright/test';

test.describe('Autonomous Voice Dispatch Mobile UI', () => {
  test('Owner can view and approve a voice dispatch proposal', async ({ page }) => {
    // Navigate to the voice dispatch page
    await page.goto('/voice-dispatch');

    // 1. Verify the mobile lock screen notification is present
    const notification = page.locator('text=Carlos, new pipe repair request from Sarah for tomorrow 2PM. $50 deposit collected.');
    await expect(notification).toBeVisible();

    // 2. Verify the main approval screen header
    const header = page.locator('h1:has-text("Booking Proposal")');
    await expect(header).toBeVisible();

    // 3. Verify the AI call summary text
    const summaryText = page.locator('text="Customer Sarah called needing a pipe repair. I quoted $150 and scheduled for Tomorrow at 2:00 PM. A $50 deposit link was sent via SMS."');
    await expect(summaryText).toBeVisible();

    // 4. Verify proposed details
    await expect(page.locator('text=Pipe Repair')).toBeVisible();
    await expect(page.locator('text=Sarah Jenkins')).toBeVisible();
    await expect(page.locator('text=Tomorrow, 2:00 PM')).toBeVisible();
    await expect(page.locator('text=Paid ($50)')).toBeVisible();

    // 5. Interact with the audio player
    const playButton = page.locator('button:has(svg.lucide-play)');
    await expect(playButton).toBeVisible();
    await playButton.click();

    // Play button should become pause button
    const pauseButton = page.locator('button:has(svg.lucide-pause)');
    await expect(pauseButton).toBeVisible();

    // 6. Approve the booking
    const approveButton = page.locator('button:has-text("Approve Route & Send Confirmation")');
    await expect(approveButton).toBeVisible();
    await expect(approveButton).toBeEnabled();

    await approveButton.click();

    // 7. Verify the button changes to confirmed state
    const confirmedButton = page.locator('button:has-text("Confirmed & Scheduled")');
    await expect(confirmedButton).toBeVisible();
    await expect(confirmedButton).toBeDisabled();
  });
});
