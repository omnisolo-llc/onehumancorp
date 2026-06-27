import { test, expect } from './fixtures';

test.describe('Shift Scheduling CUJ', () => {
  test('Manager approves a shift reassignment from a staff call-out SMS', async ({ page, loginAs, adminUser, request }) => {
    await loginAs(page, adminUser);

    // 1. Simulate the staff member sending an SMS calling out sick.
    // This goes through the Twilio webhook and gets processed by the message_triage_worker.
    await request.post('/api/v1/webhooks/twilio', {
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded'
      },
      data: 'From=whatsapp%3A%2B14155238886&To=whatsapp%3A%2B1234567890&Body=Im+sick+and+cant+make+my+shift+tomorrow'
    });

    // Wait for the background queue worker to pick it up and process it
    await page.waitForTimeout(5000);

    // 2. The manager logs into OHC and goes to the Team feed
    await page.goto('/team');
    await page.getByText('The Ambassador').click();

    // 3. Manager sees the Action Card for the shift reassignment
    const shiftCard = page.getByTestId('shift-reassignment-card').first();
    await expect(shiftCard).toBeVisible({ timeout: 15000 });

    // Verify it correctly parsed the intent and drafted a proposal
    await expect(shiftCard).toContainText('Staff Call-Out');
    await expect(shiftCard).toContainText('Im sick and cant make my shift tomorrow');
    await expect(shiftCard).toContainText('Alex is available');

    // 4. Manager taps "Approve & Notify"
    const approveBtn = page.getByTestId('approve-shift-reassignment').first();
    await approveBtn.waitFor({ state: 'visible' });
    await approveBtn.click();

    // 5. Optimistic UI update should remove the card from the feed
    await expect(shiftCard).toHaveCount(0, { timeout: 5000 });
  });
});
