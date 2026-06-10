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

    // Verify the viral loop footer
    const poweredByLink = page.getByRole('link', { name: /Powered by OHC/i });
    await expect(poweredByLink).toBeVisible();

    const onboardingHref = await poweredByLink.getAttribute('href');
    expect(onboardingHref).toContain('/onboarding?ref=');
    expect(onboardingHref).toContain('source=invoice_generator');
  });
});
