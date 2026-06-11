import { test, expect } from './fixtures';

test.describe('Growth Cloud Bridge Loop', () => {
  test('User can generate and copy a cloud bridge invite link', async ({ page }) => {
    // Navigate to the dashboard
    await page.goto('/api/ui/dashboard.html');

    // Ensure the dashboard has loaded
    await expect(page.locator('text=Dashboard').first()).toBeVisible();

    // Verify the Grow Your Team section is present
    await expect(page.locator('text=Grow Your Team')).toBeVisible();

    // Click the generate link button
    const generateBtn = page.locator('#generate-link-btn');
    await expect(generateBtn).toBeVisible();
    await generateBtn.click();

    // The container should become visible
    const linkContainer = page.locator('#link-container');
    await expect(linkContainer).toBeVisible();

    // The link should not be the fallback or error link
    const linkInput = page.locator('#referral-link');
    await expect(linkInput).toHaveValue(/https:\/\/ohc\.app\/invite\//);

    // The copy button should work
    const copyBtn = page.locator('#copy-btn');
    await expect(copyBtn).toBeVisible();

    // We can't easily assert clipboard contents in headless mode cross-platform without permissions,
    // but we can verify the button text changes indicating success.
    await copyBtn.click();
    await expect(copyBtn).toHaveText('Copied!');
  });
});
