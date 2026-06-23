import { test, expect } from './fixtures';
import { adminPage } from './fixtures';

test.describe('Growth: Footer Branding Loop Generator', () => {
  test('creates a footer branding snippet and shows soft paywall for removing branding', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard.html');

    // Click on the new Footer Branding Loop generator link
    await page.click('id=footer-branding-loop-link');

    // Verify we are on the generator page
    await expect(page).toHaveURL(/.*footer-branding-loop\.html/);

    // Verify title and description
    await expect(page.locator('h1')).toHaveText('Footer Branding Loop Builder');
    await expect(page.locator('text=Generate a "Powered by OHC" snippet')).toBeVisible();

    // Verify default state
    await expect(page.locator('id=badge-style')).toHaveValue('pill');
    await expect(page.locator('id=badge-text')).toHaveValue('powered');
    await expect(page.locator('id=remove-branding')).not.toBeChecked();

    // Verify live preview reflects default state
    await expect(page.locator('id=preview-badge-pill')).toBeVisible();
    await expect(page.locator('id=preview-text-pill')).toHaveText('Powered by OHC');
    await expect(page.locator('id=preview-badge-footer')).not.toBeVisible();

    // Interact with form
    await page.selectOption('id=badge-style', 'footer');
    await page.selectOption('id=badge-text', 'made');

    // Verify live preview updates
    await expect(page.locator('id=preview-badge-pill')).not.toBeVisible();
    await expect(page.locator('id=preview-badge-footer')).toBeVisible();
    await expect(page.locator('id=preview-text-footer')).toHaveText('Made with OHC');

    // Open embed code modal
    await page.click('id=get-code-btn');
    await expect(page.locator('id=embed-modal')).toHaveClass(/active/);

    // Verify generated code
    const code = await page.inputValue('id=embed-code');
    expect(code).toContain('footer-branding/embed.js');
    expect(code).toContain('tenant=');
    expect(code).toContain('style=footer');
    expect(code).toContain('text=Made%20with%20OHC');

    // Close modal
    await page.click('id=close-embed-btn');
    await expect(page.locator('id=embed-modal')).not.toHaveClass(/active/);

    // Test the soft paywall
    // Since the test admin user probably does not have 'has_pro' set in localStorage natively
    // checking "Remove Branding" should trigger the paywall
    await page.check('id=remove-branding');

    // Paywall should appear
    await expect(page.locator('id=paywall-modal')).toHaveClass(/active/);
    await expect(page.locator('text=Upgrade to Pro')).toBeVisible();
    await expect(page.locator('id=share-to-unlock-btn')).toBeVisible();

    // Verify checkbox is still unchecked
    await expect(page.locator('id=remove-branding')).not.toBeChecked();

    // Close paywall
    await page.click('id=close-paywall');
    await expect(page.locator('id=paywall-modal')).not.toHaveClass(/active/);
  });
});
