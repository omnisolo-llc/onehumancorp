import { test, expect } from '../../../../e2e/fixtures';

test.describe('Work Intake Widget', () => {
  test('generates and updates embed code with iframe and triggers soft paywall', async ({ memberPage }) => {
    // 1. Navigate to the page
    await memberPage.goto('/work-intake-widget');

    // Check if the page loaded
    await expect(memberPage.locator('h2', { hasText: 'Widget Configuration' })).toBeVisible();
    await expect(memberPage.locator('h2', { hasText: 'Live Preview' })).toBeVisible();

    // 2. Modify form settings
    const titleInput = memberPage.getByTestId('input-form-title');
    await titleInput.fill('Inquire Now');

    const tenantInput = memberPage.getByTestId('input-tenant-id');
    await tenantInput.fill('my-real-tenant');

    // 3. Test Soft Paywall (Growth mechanism)
    const removeBrandingCheckbox = memberPage.getByTestId('input-remove-branding');
    await removeBrandingCheckbox.check();

    const paywallModal = memberPage.getByTestId('paywall-modal');
    await expect(paywallModal).toBeVisible();

    // Close paywall
    await memberPage.getByTestId('btn-close-paywall').click();
    await expect(paywallModal).not.toBeVisible();

    // 4. Verify preview shows branding (since we didn't upgrade)
    const preview = memberPage.getByTestId('widget-preview');
    await expect(preview).toContainText('Powered by OHC');
    await expect(preview).toContainText('Inquire Now');

    // 5. Open Modal & Verify code block updates (Iframe approach)
    await memberPage.getByTestId('btn-get-code').click();

    const codeBlock = memberPage.getByTestId('embed-code-block');
    const codeText = await codeBlock.textContent();
    expect(codeText).toContain('iframe');
    expect(codeText).toContain('title=Inquire%20Now');
    expect(codeText).toContain('tenant=my-real-tenant');
    expect(codeText).toContain('ref=my-real-tenant'); // Viral loop link

    // 6. Test copy button (checking if it exists and works)
    const copyBtn = memberPage.getByTestId('btn-copy-code');
    await expect(copyBtn).toBeVisible();
    await copyBtn.click();

    // Close modal
    await memberPage.getByRole('button', { name: 'Close embed modal' }).click();

    // Ensure mobile view works without breaking
    await memberPage.setViewportSize({ width: 375, height: 812 });
    await expect(memberPage.locator('h2', { hasText: 'Widget Configuration' })).toBeVisible();
  });
});
