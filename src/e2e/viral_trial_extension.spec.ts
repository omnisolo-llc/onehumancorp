import { expect, test } from './fixtures';

test.describe('Viral Trial Extension Loop', () => {
  test('should display the Earn a Free Month banner, generate an invite link, and simulate extension', async ({ page }) => {
    // Navigate to pricing page
    await page.goto('/pricing');

    // Wait for the banner to be visible
    const bannerHeading = page.getByRole('heading', { name: 'Earn a Free Month of Pro' });
    await expect(bannerHeading).toBeVisible();

    // Verify the "Get My Invite Link" button is there
    const getLinkBtn = page.getByRole('button', { name: 'Get My Invite Link' });
    await expect(getLinkBtn).toBeVisible();

    // Click to generate link
    await getLinkBtn.click();

    // The button should be replaced by an input box with the link and a Copy button
    const linkInput = page.locator('input[readonly]');
    await expect(linkInput).toBeVisible();
    await expect(linkInput).toHaveValue(/https:\/\/ohc\.store\/join\?ref=.+/);

    // Verify the 'Trial extension unlocked!' text is visible after API call
    const unlockedText = page.getByText('Trial extension unlocked!');
    await expect(unlockedText).toBeVisible();

    // Test the copy button
    const copyBtn = page.getByRole('button', { name: 'Copy' });
    await expect(copyBtn).toBeVisible();
    await copyBtn.click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();
  });
});
