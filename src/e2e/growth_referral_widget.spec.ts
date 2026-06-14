import { test, expect } from '@playwright/test';

// Skipped because Next.js UI is legacy/prototype.
// Widget component logic is covered by GrowthReferralWidget.test.tsx
// and dashboard API errors prevent E2E execution without robust backend setup.
test.describe('Growth Referral Widget', () => {
  test('generates and copies link correctly', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('http://localhost:3000/dashboard');

    // Ensure the widget is visible
    const getLinkButton = page.getByRole('button', { name: 'Get My Invite Link' });
    await expect(getLinkButton).toBeVisible();

    // Click to generate link
    await getLinkButton.click();

    // Verify generated link appears
    const linkInput = page.getByTestId('dashboard-viral-invite-widget').getByRole('textbox');
    await expect(linkInput).toBeVisible();

    // Verify action buttons appear
    const copyButton = page.getByRole('button', { name: 'Copy', exact: true });
    await expect(copyButton).toBeVisible();
    await expect(page.locator('text=Share on WhatsApp')).toBeVisible();

    // Give clipboard permissions
    await page.context().grantPermissions(['clipboard-read', 'clipboard-write']);

    // Test Copy functionality
    await copyButton.click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();

    // Verify clipboard content
    const clipboardText = await page.evaluate(() => navigator.clipboard.readText());
    expect(clipboardText.length).toBeGreaterThan(0);
  });
});
