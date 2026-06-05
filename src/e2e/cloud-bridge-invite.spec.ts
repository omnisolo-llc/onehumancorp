import { test, expect } from '@playwright/test';

test.describe('Cloud Bridge Invite Loop', () => {
  test('Invite Modal opens and the landing page works correctly', async ({ page }) => {
    // Navigate to the Team page (use relative path so baseURL applies)
    await page.goto('/team');

    // Wait for the "Invite to Cloud Team" button to be visible and click it
    const inviteButton = page.locator('button:has-text("Invite to Cloud Team")');
    await expect(inviteButton).toBeVisible();
    await inviteButton.click();

    // Verify the modal is shown
    const modalHeader = page.locator('h2:has-text("Cloud Bridge Invite")');
    await expect(modalHeader).toBeVisible();

    // Get the invite link from the input
    const inviteLinkInput = page.locator('input#cloud-bridge-invite-link');
    await expect(inviteLinkInput).toBeVisible();
    const linkValue = await inviteLinkInput.inputValue();

    // Ensure the link is properly formatted (e.g. contains /invite/)
    expect(linkValue).toContain('/invite/');

    // Navigate to the invite link (ensure it uses the current base URL for relative routing in tests)
    const relativeLink = new URL(linkValue).pathname;
    await page.goto(relativeLink);

    // Verify the landing page shows the correct UI elements
    const inviteHeader = page.locator('h1:has-text("You\'ve been invited")');
    await expect(inviteHeader).toBeVisible();

    const securityBadge = page.locator('text=Zero Data Leakage Guaranteed');
    await expect(securityBadge).toBeVisible();

    // Click the "Accept Invitation & Join Team Workspace" button
    const joinButton = page.locator('button:has-text("Accept Invitation & Join Team Workspace")');
    await expect(joinButton).toBeVisible();
    await joinButton.click();

    // Verify the button text changes to "Provisioning Tenant..."
    const provisioningText = page.locator('button:has-text("Provisioning Tenant...")');
    await expect(provisioningText).toBeVisible();

    // Wait for redirection to the dashboard
    await page.waitForURL('**/dashboard?joined=true', { timeout: 5000 });
    expect(page.url()).toContain('/dashboard?joined=true');
  });
});
