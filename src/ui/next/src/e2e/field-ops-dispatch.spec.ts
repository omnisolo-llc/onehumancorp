import { test, expect } from '../../../e2e/fixtures';

// Emulate mobile viewport for this test block
test.use({ viewport: { width: 375, height: 667 } });

test.describe('Zero-Touch Smart Service Dispatch & Route Optimization Engine', () => {
  test('Carlos can report a delay and Operations Agent updates schedule automatically', async ({ page, loginAs, adminUser }) => {
    // 1. The user logs in and views their daily schedule.
    await loginAs(page, adminUser);
    await page.goto('/field-ops/jobs');

    // 2. The system has automatically inserted appropriate travel time blocks
    // Wait for the route to load and check Morning Briefing Card
    await expect(page.locator('text=Today\'s Route')).toBeVisible();
    await expect(page.locator('text=Your route today is optimized.')).toBeVisible();

    // 3. The user taps "Running Late" on the first appointment.
    const runningLateBtn = page.locator('button', { hasText: 'Running Late' }).first();
    await runningLateBtn.waitFor({ state: 'visible' });
    await runningLateBtn.click();

    // 4. The Operations Agent calculates the cascading delay and presents an Action Card.
    // Expect the Delay Intervention Card/Modal to be visible
    const modalText = page.locator('text=Drafting delay notifications for the next').first();
    await expect(modalText).toBeVisible();

    // Verify "Approve & Send" button is present
    const approveBtn = page.locator('button', { hasText: 'Approve & Send' });
    await expect(approveBtn).toBeVisible();

    // 5. The user taps "Approve", and the schedule updates while the notifications are sent.
    await approveBtn.click();

    // Expect the UI to show that the action was taken
    const successMsg = page.locator('text=Notified').first();
    await expect(successMsg).toBeVisible();
  });
});
