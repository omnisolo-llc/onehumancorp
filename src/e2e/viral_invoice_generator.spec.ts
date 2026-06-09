import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('viral_invoice_generator');

test.describe('Viral Invoice Generator Loop', () => {
  test('should allow creating an invoice and viewing the viral loop', async ({ page }) => {
    // Navigate to dashboard first to find the link
    await page.goto('/dashboard');

    const invoiceLink = page.locator('a[href="/invoice-generator"]');
    await expect(invoiceLink).toBeVisible();
    await invoiceLink.click();

    // Verify page content
    await expect(page.getByRole('heading', { name: 'Invoice Generator' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Create Professional Invoice' })).toBeVisible();

    // Fill out the form
    await page.fill('input[placeholder="e.g. Acme Corp"]', 'Globex Corporation');
    await page.fill('textarea[placeholder="e.g. Website Redesign and SEO Optimization"]', 'Consulting Services for Q3');
    await page.fill('input[placeholder="e.g. 1500.00"]', '2500');

    // Ensure we start without Pro
    await page.evaluate(() => {
        window.localStorage.setItem('has_pro', 'false');
    });

    // Verify soft paywall loop for white-label invoice
    const removeBrandingCheckbox = page.locator('input[type="checkbox"]');
    await removeBrandingCheckbox.click({ force: true }); // Using force because it's sr-only peer

    // Verify soft paywall modal appears
    const paywallHeading = page.getByRole('heading', { name: 'Upgrade to Pro' });
    await expect(paywallHeading).toBeVisible();

    const shareToUnlockBtn = page.getByRole('button', { name: 'Share to get 7 Days Pro' });
    await expect(shareToUnlockBtn).toBeVisible();

    // Mock window.open before clicking to prevent real popups
    await page.evaluate(() => {
        window.open = function() { return window; };
    });

    // Click Share to unlock (which grants Pro and sets removeBranding to true)
    await shareToUnlockBtn.click();

    // Verify modal closes
    await expect(paywallHeading).toBeHidden();

    // Verify the toggle is now checked
    await expect(removeBrandingCheckbox).toBeChecked();

    // Click generate
    const generateBtn = page.getByRole('button', { name: 'Generate Shareable Invoice' });
    await expect(generateBtn).toBeVisible();
    await generateBtn.click();

    // Verify the invoice is ready
    await expect(page.getByRole('heading', { name: 'Your Invoice is Ready!' })).toBeVisible();

    // Click preview invoice
    const previewLink = page.getByRole('link', { name: 'Preview Invoice' });
    await expect(previewLink).toBeVisible();

    // Instead of waiting for a new tab, let's navigate the current page to the href
    const href = await previewLink.getAttribute('href');
    expect(href).toContain('/invoice-generator/view?data=');

    await page.goto(href!);

    // Verify the invoice view
    await expect(page.getByRole('heading', { name: 'INVOICE' })).toBeVisible();
    await expect(page.getByText('Globex Corporation')).toBeVisible();
    await expect(page.getByText('Consulting Services for Q3')).toBeVisible();
    await expect(page.getByText('$2500.00')).toBeVisible();

    // Verify the viral loop footer is NOT visible because we removed branding
    const poweredByLink = page.getByRole('link', { name: /Powered by OHC/i });
    await expect(poweredByLink).toBeHidden();
  });
});
