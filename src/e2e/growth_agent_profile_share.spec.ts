import { test, expect } from './fixtures';

test.describe('Agent Profile Cloud Bridge Share', () => {
  test('User can generate and copy a cloud bridge invite link from the agent profile', async ({ page }) => {
    // Navigate to the agent profile page
    await page.goto('/api/ui/agent-profile.html');

    // Ensure the page has loaded by checking for the branding link or standard text
    await expect(page.locator('text=Powered by OHC').first()).toBeVisible();

    // Verify the Share Profile to Cloud Team button is present
    const shareBtn = page.locator('#cloud-bridge-share-btn');
    await expect(shareBtn).toBeVisible();
    await expect(shareBtn).toHaveText('Share Profile to Cloud Team');

    // Click the share button
    await shareBtn.click();

    // The button should temporarily say Generating... but eventually hide,
    // and the container should become visible
    const shareContainer = page.locator('#cloud-bridge-share-container');
    await expect(shareContainer).toBeVisible();

    // The link should not be empty and should have the correct format
    const shareInput = page.locator('#cloud-bridge-invite-link');
    await expect(shareInput).toHaveValue(/https:\/\/ohc\.app\/invite\/.+/);

    // The copy button should work
    const copyBtn = page.locator('#cloud-bridge-copy-btn');
    await expect(copyBtn).toBeVisible();

    // Verify button text changes to Copied! after clicking
    await copyBtn.click();
    await expect(copyBtn).toHaveText('Copied!');
  });
});
