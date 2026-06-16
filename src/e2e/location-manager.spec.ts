import { test, expect } from '@playwright/test';

test.describe('Location Manager Coordination & Escalation Workflow', () => {
  test('Jun can view local alerts, draft an AI escalation, and escalate to Owner', async ({ page }) => {
    // Navigate to the location manager view
    await page.goto('/locations.html');

    // Wait for the page to load
    await expect(page.locator('h1')).toHaveText('Location Manager');

    // Verify Active Alerts section exists
    await expect(page.locator('text=Active Alerts & Tasks')).toBeVisible();

    // Verify the specific alert is present
    const alertCard = page.locator('#alert-1');
    await expect(alertCard).toBeVisible();
    await expect(alertCard.locator('.card-title')).toHaveText('Pickup Delays Spike');

    // Click "Escalate to Owner"
    await alertCard.locator('button:has-text("Escalate to Owner")').click();

    // Verify the AI draft form appears
    const escForm = page.locator('#esc-form-alert-1');
    await expect(escForm).toBeVisible();
    await expect(escForm.locator('.ai-draft-badge')).toBeVisible();

    // Verify draft text is populated
    const draftTextarea = escForm.locator('textarea');
    await expect(draftTextarea).toHaveValue(/Location A is experiencing a 30% increase/);

    // Edit the text (simulate Jun editing)
    await draftTextarea.fill('Location A is experiencing a 30% increase in wait times. Staffing appears adequate, but the POS terminal is offline. Requesting IT support.');

    // Click "Send to Owner"
    await escForm.locator('button:has-text("Send to Owner")').click();

    // Verify the alert card disappears from Active Alerts
    await expect(alertCard).not.toBeVisible();

    // Verify the escalation appears in the Pending Escalations list (Owner View)
    const pendingList = page.locator('#escalations-list');
    await expect(pendingList.locator('.card-title', { hasText: 'Pickup Delays Spike' })).toBeVisible();

    // Verify the status is "Pending Review"
    await expect(pendingList.locator('.status-badge')).toHaveText('Pending Review');

    // Verify the edited text is displayed in the pending card
    await expect(pendingList.locator('text="Location A is experiencing a 30% increase in wait times. Staffing appears adequate, but the POS terminal is offline. Requesting IT support."')).toBeVisible();
  });
});
