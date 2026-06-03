import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

// Using the fallback approach, this just indicates that the tests ran locally in a real browser.
currentAppSmoke('viral_growth_loops');

test.describe('Viral Growth Loop - Cloud Bridge Invite', () => {
  test('generates dynamic referral link via API proxy on the Team page', async ({ page, context }) => {
    // Navigate to the Team page
    await page.goto('/team');
    await page.waitForLoadState('networkidle');

    // Ensure the Team page header is loaded
    await expect(page.locator('h1', { hasText: 'Your Team' }).first()).toBeVisible({ timeout: 15000 });

    // Open the Cloud Bridge Invite modal by clicking "Invite to Cloud Team"
    const inviteButton = page.locator('button', { hasText: 'Invite to Cloud Team' });
    await expect(inviteButton).toBeVisible();
    await inviteButton.click();

    // Verify modal is shown
    const modalHeader = page.locator('h2', { hasText: 'Cloud Bridge Invite' });
    await expect(modalHeader).toBeVisible();

    // Wait for the link generation to complete (the input value should start with https://ohc.app/ref/)
    const inviteLinkInput = page.locator('input#cloud-bridge-invite-link');
    await expect(inviteLinkInput).toBeVisible();

    // We expect the proxy to hit the rust backend and return a ref URL starting with https://ohc.app/ref/
    await expect(inviteLinkInput).toHaveValue(/https:\/\/ohc\.app\/ref\/.+/, { timeout: 10000 });

    // Click the Copy Link button and intercept clipboard text
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);

    const copyBtn = page.locator('button', { hasText: 'Copy Link' });
    await expect(copyBtn).toBeVisible();
    await copyBtn.click();

    // Verify it changes to "Copied!"
    await expect(page.locator('button', { hasText: 'Copied!' })).toBeVisible();

    // Read clipboard and ensure the correct URL was copied
    const clipboardText = await page.evaluate(() => navigator.clipboard.readText());
    expect(clipboardText).toMatch(/^https:\/\/ohc\.app\/ref\/.+/);
  });
});
