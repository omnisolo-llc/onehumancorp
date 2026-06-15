import { test, expect } from './fixtures';

test.describe('Success Milestone Widget', () => {
  test('displays milestone and handles share', async ({ page, loginAs, adminUser }) => {
    // Navigate to the Next.js dashboard
    await page.goto('http://localhost:3000/dashboard');

    // Wait for the Milestone component to appear
    const milestoneWidget = page.locator('text=100th Order Delivered!');
    await expect(milestoneWidget).toBeVisible({ timeout: 15000 });

    // Verify copy button
    const copyBtn = page.getByRole('button', { name: /Copy & Share to Unlock/i });
    await expect(copyBtn).toBeVisible();

    // Verify share on X link
    const xLink = page.getByRole('link', { name: /Share on X/i });
    await expect(xLink).toBeVisible();
    await expect(xLink).toHaveAttribute('href', /twitter\.com\/intent\/tweet/);

    // Mock clipboard
    await page.context().grantPermissions(['clipboard-read', 'clipboard-write']);

    // Click the copy button
    await copyBtn.click();
    await expect(page.locator('text=Copied to Clipboard!')).toBeVisible();

    const clipboardText = await page.evaluate('navigator.clipboard.readText()');
    expect(clipboardText).toContain('I just hit my 100th order using OHC');
  });
});
