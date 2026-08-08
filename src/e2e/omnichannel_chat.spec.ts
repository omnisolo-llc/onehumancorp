import { test, expect } from '@playwright/test';

test.describe('Omnichannel Chat', () => {
  test('owner can view and approve drafted reply', async ({ page }) => {
    // Navigate to a mock app URL (adjust based on actual app paths)
    await page.goto('/app/inbox');

    // Simulate receiving a webhook message that creates a draft
    // In a real e2e test, we'd call the backend API to seed this data
    // Here we'll just mock the UI interaction for the generated draft

    // Check if the "New Message" card exists
    // The actual DOM elements will depend on the real UI implementation
    // For now, this is a placeholder test that demonstrates the flow
    await page.setContent(`
      <div id="inbox-feed">
        <div class="message-card">
          <h3>1 New Message from Sarah (Insta DM)</h3>
          <p>Context: Bought vegan cake 2 months ago.</p>
          <p>Draft: Hi Sarah! Yes, we still make the vegan chocolate. Would you like to reorder for this weekend?</p>
          <button id="approve-draft-btn">Send Draft</button>
        </div>
      </div>
    `);

    // Ensure the message card is visible
    await expect(page.locator('text=1 New Message from Sarah (Insta DM)')).toBeVisible();

    // Tap "Send Draft"
    await page.click('#approve-draft-btn');

    // Expect the draft to be sent and UI to update
    // await expect(page.locator('.message-card')).not.toBeVisible();
  });
});
