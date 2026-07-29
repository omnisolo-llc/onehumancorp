import { test, expect } from './fixtures';

test.describe('Work-Intake Widget Viral Loop', () => {
  test('should display the soft paywall modal and handle share bypass', async ({ page, request, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/work-intake-widget');

    // 1. Verify the page header
    await expect(page.getByRole('heading', { name: 'Work-Intake Widget 📋' })).toBeVisible();

    // 2. Click the remove branding checkbox (Note: it is a test checkbox that opens a modal, so use .click())
    const removeBrandingCheckbox = page.getByLabel('Remove "Powered by OHC" branding');
    await removeBrandingCheckbox.click();

    // 3. Verify the soft paywall modal appears
    const modalHeading = page.getByRole('heading', { name: 'Upgrade to Pro' });
    await expect(modalHeading).toBeVisible();
    await expect(page.getByText('Make the Work Intake Widget 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.')).toBeVisible();

    // 4. Test window.open to prevent opening a new tab
    await page.evaluate(() => {
        window.open = function() { return window; };
    });

    // 5. Click the share button to trigger the bypass API call
    const shareButton = page.getByRole('button', { name: /Share on X to Unlock/i });
    await expect(shareButton).toBeVisible();
    await shareButton.click();

    // 6. Verify the loading state
    await expect(page.getByText(/Verifying Share.../i)).toBeVisible();

    // 7. Verify the success state and modal disappearance
    await expect(modalHeading).not.toBeVisible({ timeout: 5000 });

    // 8. Verify the branding is actually removed in the code snippet
    const getCodeBtn = page.getByRole('button', { name: 'Get Widget Code' });
    await getCodeBtn.click();
    const textarea = page.locator('textarea');
    const codeValue = await textarea.inputValue();
    expect(codeValue).not.toContain('Powered by OHC');
  });
});
