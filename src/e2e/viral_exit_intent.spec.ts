import { test, expect } from './fixtures';

test.describe('Viral Exit-Intent Loop', () => {
  test('should allow owner to create an exit intent popup and copy code', async ({ page, context }) => {
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);

    // 1. Navigate to dashboard
    await page.goto('/dashboard.html');

    // 2. Find and click the Exit-Intent link
    const exitIntentLink = page.locator('a[href="exit-intent-builder.html"]');
    await expect(exitIntentLink).toBeVisible();
    await exitIntentLink.click();

    // Verify page content
    await expect(page.getByRole('heading', { name: 'Exit-Intent Pop-up Builder' })).toBeVisible();
    await expect(page.getByText('Recover abandoning visitors')).toBeVisible();

    // 3. Edit input fields
    await page.getByPlaceholder('Wait! Before you go...').fill('Special Limited Time Offer!');
    await page.getByPlaceholder('Get 10% off your first order...').fill('Get 10% off your first order when you sign up for our newsletter.');

    // 4. Assert Live Preview reflects changes
    await expect(page.locator('.preview-popup').filter({ hasText: 'Special Limited Time Offer!' })).toBeVisible();
    await expect(page.locator('.preview-popup').filter({ hasText: 'Get 10% off your first order when you sign up for our newsletter.' })).toBeVisible();

    // 5. Test Copy Embed Code logic and Viral Referral Link
    const copyButton = page.getByRole('button', { name: 'Copy to Clipboard' });
    await copyButton.click();
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();

    // Check the embed code text area content directly since we don't have a reliable way to check clipboard in all CI environments
    const embedCode = await page.locator('#code-output').textContent();
    expect(embedCode).toContain('/api/v1/growth/referrals/click?target=/setup.html&ref=');
    expect(embedCode).toContain('⚡ Powered by OHC');

    // 6. Test Paywall logic
    const brandingToggle = page.locator('#branding-toggle');
    await brandingToggle.click({force: true}); // The slider overlaps it

    // Ensure the paywall modal opens
    const upgradeButton = page.getByRole('button', { name: 'Upgrade to Pro' });
    await expect(upgradeButton).toBeVisible();

    // Click "Upgrade to Pro" to close modal and simulate upgrade state
    await upgradeButton.click();

    // Ensure modal closes and toggle switches to on
    await expect(upgradeButton).toBeHidden();
    await expect(brandingToggle).toBeChecked();

    // Verify branding is removed from preview
    await expect(page.locator('#prev-branding')).toBeHidden();

    // 7. Verify "Back to Dashboard" footer link
    const backLink = page.locator('a.back-link', { hasText: 'Back to Dashboard' });
    await expect(backLink).toBeVisible();
    await backLink.click();

    // Verify we're back
    await expect(page.locator('a[href="exit-intent-builder.html"]')).toBeVisible();
  });
});
