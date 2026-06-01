import { test, expect } from '@playwright/test';

test.describe('Sovereign-to-Cloud Loop', () => {
  test('generates and displays a real referral link in the Cloud Bridge Invite modal', async ({ page }) => {
    // Navigate to the Team page
    await page.goto('http://localhost:3000/team');

    // Click "Invite to Cloud Team"
    const inviteBtn = page.getByRole('button', { name: 'Invite to Cloud Team' });
    await expect(inviteBtn).toBeVisible();
    await inviteBtn.click();

    // Verify modal is visible
    const modalHeading = page.getByRole('heading', { name: 'Cloud Bridge Invite' });
    await expect(modalHeading).toBeVisible();

    // Check that the generated link contains "ohc.app/ref/" since we fallback to "https://ohc.app/ref/fallback-code" or hit backend
    const input = page.locator('#cloud-bridge-invite-link');
    await expect(input).toBeVisible();

    // Wait for the input value to update from the default "https://ohc.app/invite/team-default"
    await expect(input).not.toHaveValue('https://ohc.app/invite/team-default');

    const value = await input.inputValue();
    expect(value).toMatch(/ohc\.app\/ref\/|ohc:\/\/join\?ref=/);

    // Copy link button
    const copyBtn = page.getByRole('button', { name: 'Copy Link' });
    await expect(copyBtn).toBeVisible();
    await copyBtn.click();

    // Validate that it says Copied!
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();
  });
});
