import { test, expect } from './fixtures';

test.describe('Work-Intake Widget Growth Loop', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the work intake widget builder page
    await page.goto('/work-intake-widget');
    await page.waitForLoadState('networkidle');
  });

  test('should display the widget builder, update live preview, and generate embed code with branding', async ({ page }) => {
    // 1. Verify the page header
    await expect(page.getByRole('heading', { name: 'Work-Intake Widget 📋' })).toBeVisible();

    // 2. Change Tenant ID
    const tenantInput = page.locator('input[placeholder="e.g. my-business"]');
    await tenantInput.fill('e2e-tenant');

    // 3. Change Form Title
    const titleInput = page.locator('input[placeholder="e.g. Work Request"]');
    await titleInput.fill('E2E Custom Request');

    // 4. Toggle dark theme
    const darkThemeBtn = page.getByRole('button', { name: 'Dark' });
    await darkThemeBtn.click();

    // 5. Verify the iframe URL updates correctly
    const iframe = page.locator('iframe');
    await expect(iframe).toHaveAttribute('src', /tenant=e2e-tenant/);
    await expect(iframe).toHaveAttribute('src', /theme=dark/);
    await expect(iframe).toHaveAttribute('src', /title=E2E%20Custom%20Request/);

    // 6. Click "Get Widget Code" to open the modal
    const getCodeBtn = page.getByRole('button', { name: 'Get Widget Code' });
    await getCodeBtn.click();

    // 7. Verify the modal appears and contains the expected code
    await expect(page.getByRole('heading', { name: 'Embed Work-Intake Widget' })).toBeVisible();

    // Check the textarea for the expected embed code
    const textarea = page.locator('textarea');
    await expect(textarea).toBeVisible();
    const codeValue = await textarea.inputValue();

    expect(codeValue).toContain('tenant=e2e-tenant');
    expect(codeValue).toContain('theme=dark');
    expect(codeValue).toContain('title=E2E%20Custom%20Request');
    expect(codeValue).toContain('Powered by OHC'); // Verify the viral loop is intact
  });

  test('should show soft paywall when attempting to remove branding', async ({ page }) => {
    // Check the remove branding checkbox
    const removeBrandingCheckbox = page.getByLabel('Remove "Powered by OHC" branding');
    await removeBrandingCheckbox.check();

    // Verify soft paywall appears
    await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).toBeVisible();

    // Check that checkbox was unchecked automatically
    expect(await removeBrandingCheckbox.isChecked()).toBeFalsy();
  });
});
