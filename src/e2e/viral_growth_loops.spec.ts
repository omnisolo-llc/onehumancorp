import { test, expect } from './fixtures';

test.describe('Viral Growth Loops - Referral', () => {
  test('Dashboard should display the viral referral loop correctly and generate a link', async ({ page }) => {
    // Navigate to the Dashboard
    await page.goto('/dashboard');

    // Wait for dashboard to load
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // Locate the "Invite a Business & Earn $50" button
    const referralButton = page.getByRole('button', { name: '🎁 Invite a Business & Earn $50' });
    await expect(referralButton).toBeVisible();

    // Click it to trigger the modal and generate link
    await referralButton.click();

    // Verify the modal text
    await expect(page.getByText('Your Unique Link')).toBeVisible();

    // Check that the referral link input contains the generated link
    const referralLinkInput = page.locator('input[readOnly]').filter({ hasText: 'ohc' }).first();
    // Wait for the fetch request to finish and link to be displayed.
    await expect(referralLinkInput).toHaveValue(/ohc\.store\/join\?ref=|https:\/\/ohc\.app\/ref\//, { timeout: 10000 });

    // Click the copy button to trigger any tracking
    const copyButton = page.getByRole('button', { name: 'Copy' }).first();
    await expect(copyButton).toBeVisible();
    await copyButton.click();
    await expect(page.getByRole('button', { name: 'Copied!' }).first()).toBeVisible();

    // Close the modal by clicking the X svg button
    const closeBtn = page.locator('button').filter({ has: page.locator('svg') }).nth(1);
    await closeBtn.click();
  });

  test('Dashboard should display Embed Your Store viral widget', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Embed Your Store' }).first()).toBeVisible();

    const embedButton = page.getByRole('button', { name: 'Get Widget' });
    await embedButton.click();

    await expect(page.getByRole('heading', { name: 'Embed Storefront' }).first()).toBeVisible();
    const copyButton = page.getByRole('button', { name: 'Copy Code' });
    await copyButton.click();
    await expect(page.getByRole('button', { name: 'Copied!' }).first()).toBeVisible();
  });
});
