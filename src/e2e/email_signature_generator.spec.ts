import { test, expect } from './fixtures';

test.describe('Email Signature Generator Viral Loop', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate directly to the new generator
    await page.goto('/email-signature-generator');
    await page.waitForLoadState('networkidle');
  });

  test('should display the signature builder, update live preview, and generate signature code with branding', async ({ page }) => {
    // 1. Verify the page header
    await expect(page.getByRole('heading', { name: 'Free Email Signature Generator' })).toBeVisible();

    // 2. Change Name
    const nameInput = page.locator('input[placeholder="e.g. Jane Doe"]');
    await nameInput.fill('John Smith');

    // 3. Change Title
    const titleInput = page.locator('input[placeholder="e.g. Founder & CEO"]');
    await titleInput.fill('Chief E2E Tester');

    // 4. Verify the preview updates correctly
    const previewContainer = page.locator('#signature-preview');
    await expect(previewContainer).toContainText('John Smith');
    await expect(previewContainer).toContainText('Chief E2E Tester');

    // 5. Verify the viral loop backlink is present
    await expect(previewContainer).toContainText('One Human Corp');
    await expect(previewContainer.locator('a[href*="api/v1/growth/referrals/click"]')).toBeVisible();

    // 6. Click "Copy Signature HTML" button
    const copyBtn = page.getByRole('button', { name: 'Copy Signature HTML' });
    await expect(copyBtn).toBeVisible();
    // We won't test clipboard here because of browser security contexts in CI,
    // but we can ensure the button is interactive.
  });

  test('should show soft paywall when attempting to remove branding', async ({ page }) => {
    // Check the remove branding checkbox
    const removeBrandingCheckbox = page.getByLabel('Remove "Powered by OHC" branding');
    await removeBrandingCheckbox.click();

    // Verify soft paywall appears
    await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).toBeVisible();

    // Check that checkbox was unchecked automatically (it should be set to false when soft paywall is opened)
    expect(await removeBrandingCheckbox.isChecked()).toBeFalsy();
  });
});
