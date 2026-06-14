import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('viral_digital_business_card', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'viral_digital_business_card');
});

test.describe('Viral Digital Business Card Growth Loop', () => {
  test('should allow creating a digital business card, checking the preview, and copying the link', async ({ page, context }) => {
    // Grant clipboard permissions for copying the link
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);

    await page.goto('/digital-business-card');

    // Wait for the UI to be ready
    await page.waitForLoadState('networkidle');

    // Verify header and description
    await expect(page.getByRole('heading', { name: 'Digital Business Card' })).toBeVisible();
    await expect(page.getByText('Customize your digital business card')).toBeVisible();

    // Fill the card details
    const nameInput = page.locator('input').nth(0);
    const titleInput = page.locator('input').nth(1);
    const companyInput = page.locator('input').nth(2);

    await nameInput.fill('Sarah Builder');
    await titleInput.fill('Lead Architect');
    await companyInput.fill('Sarah Design Studio');

    // Verify preview updates
    await expect(page.getByRole('heading', { name: 'Sarah Builder' })).toBeVisible();
    await expect(page.getByText('Lead Architect')).toBeVisible();
    await expect(page.getByText('Sarah Design Studio')).toBeVisible();

    // Verify "Powered by OHC" branding is visible by default
    const brandingLink = page.locator('a:has-text("⚡ Powered by OHC")');
    await expect(brandingLink).toBeVisible();
    await expect(brandingLink).toHaveAttribute('href', /api\/v1\/growth\/referrals\/click/);

    // Click "Copy Share Link"
    const copyBtn = page.getByRole('button', { name: 'Copy Share Link' });
    await copyBtn.click();

    // Verify button text changes to Copied!
    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();

    // Verify clipboard content
    const clipboardText = await page.evaluate("navigator.clipboard.readText()");
    expect(clipboardText).toContain('https://ohc.app/card/');

    // Test the soft paywall for removing branding
    const removeBrandingCheckbox = page.locator('input[type="checkbox"]');
    await removeBrandingCheckbox.check();

    // Verify soft paywall appears
    const paywallHeading = page.getByRole('heading', { name: 'Upgrade to Pro' });
    await expect(paywallHeading).toBeVisible();
    await expect(page.getByText('Make your Digital Business Card 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.')).toBeVisible();

    // Close paywall via "Share on X to get 7 Days Free"
    // To prevent new tab from actually opening during test, mock window.open
    await page.evaluate(() => {
        window.open = function() { return window; };
    });
    const trialExtensionBtn = page.getByRole('button', { name: /Share on X to get 7 Days Free/ });
    await trialExtensionBtn.click();

    // Soft paywall should close, and branding should be removed
    await expect(paywallHeading).toBeHidden();
    await expect(brandingLink).toBeHidden();
  });
});
