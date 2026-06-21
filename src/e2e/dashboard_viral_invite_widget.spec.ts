import { test, expect } from './fixtures';

test.describe('Dashboard Viral Invite Widget', () => {
  test('should display the widget with loading state and invite link', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Wait for the widget to appear
    const widget = page.locator('[data-testid="dashboard-viral-invite-widget"]');
    await expect(widget).toBeVisible();

    // Check initial state
    const generateBtn = page.locator('#dashboard-invite-btn');
    await expect(generateBtn).toBeVisible();
    await expect(generateBtn).toContainText('Get My Invite Link');

    // Click generate button
    await generateBtn.click();

    // It should briefly show loading
    // Then show the input container
    const inviteContainer = page.locator('#dashboard-invite-container');
    await expect(inviteContainer).toBeVisible({ timeout: 10000 });

    // Verify link structure
    const inviteLinkInput = page.locator('#dashboard-invite-link');
    await expect(inviteLinkInput).toBeVisible();
    const linkValue = await inviteLinkInput.inputValue();
    expect(linkValue).toMatch(/^https?:\/\//);

    // Copy button
    const copyBtn = page.locator('#dashboard-copy-btn');
    await expect(copyBtn).toBeVisible();
  });
});
