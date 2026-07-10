import { test, expect } from '@playwright/test';

test.describe('Assistant Agentic Workflow', () => {
  test('User can request an action, see a proposed card, and approve it', async ({ page }) => {
    // Navigate to the assistant
    await page.goto('/assistant.html');

    // Ensure the textarea is present
    const composer = page.locator('textarea[aria-label="Composer Input"]');
    await expect(composer).toBeVisible();

    // Type the request
    await composer.fill('Add a new chocolate cake');
    await composer.press('Enter');

    // Wait for the response and the proposed action card
    // The agent will first append "Agent is thinking...", then "I have drafted a plan...", then the card
    const proposedActionTitle = page.locator('h3', { hasText: 'Proposed Action' });
    await expect(proposedActionTitle).toBeVisible({ timeout: 10000 });

    // Verify the description and buttons
    await expect(page.locator('text=Task routed via semantic gateway')).toBeVisible();

    const approveBtn = page.locator('button', { hasText: 'Approve & Execute' });
    await expect(approveBtn).toBeVisible();

    const cancelBtn = page.locator('button', { hasText: 'Cancel' });
    await expect(cancelBtn).toBeVisible();

    // Click approve
    await approveBtn.click();

    // Verify approval happened successfully
    // Look for the "Action successfully executed." string appended as a message from the assistant
    await expect(page.locator('div.message.assistant', { hasText: 'Action successfully executed.' })).toBeVisible({ timeout: 10000 });

    // Also verify the button state changes
    await expect(page.locator('button', { hasText: 'Approved' })).toBeVisible();
  });
});
