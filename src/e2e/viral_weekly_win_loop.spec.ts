import { test, expect } from './fixtures';

test.describe('Weekly Win Viral Share Growth Loop', () => {
  test('generates and displays correct share payload including Powered by OHC', async ({ page, loginAs, unlimitedAdminUser }) => {
    // Assuming we can seed data via SQL or API - the prompt requires us to use real paths.
    // The backend handles zero counts perfectly well by returning 0s.
    // So hitting the button and expecting the success text handles the "zero mock data" constraint.

    // Navigate to dashboard where the widget is embedded
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    // Locate the Weekly Win Share Widget
    const widget = page.getByTestId('weekly-win-share-widget');
    await expect(widget).toBeVisible();

    // Verify it contains the call-to-action
    await expect(widget.getByRole('heading', { name: 'Celebrate Your Week' })).toBeVisible();

    // Click the Generate Weekly Recap button
    const generateBtn = widget.getByRole('button', { name: /Generate Weekly Recap/ });
    await expect(generateBtn).toBeVisible();
    await generateBtn.click();

    // The text should now contain the generated text with the stats
    await expect(widget.getByText(/Crushed it this week using OHC!/)).toBeVisible();

    // Check that it contains the viral branding
    await expect(widget.getByText(/⚡ Powered by OHC/)).toBeVisible();

    // Verify the Share and Copy buttons are present
    const copyBtn = widget.getByRole('button', { name: 'Copy' });
    await expect(copyBtn).toBeVisible();
    await copyBtn.click();
    await expect(widget.getByRole('button', { name: 'Copied!' })).toBeVisible();

    const shareXBtn = widget.getByRole('link', { name: /Share on X/ });
    await expect(shareXBtn).toBeVisible();

    // Validate the share link contains the required viral text
    const href = await shareXBtn.getAttribute('href');
    expect(href).toContain('twitter.com/intent/tweet');
    expect(href).toContain(encodeURIComponent('⚡ Powered by OHC'));
  });
});
