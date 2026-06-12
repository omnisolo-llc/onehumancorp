import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('viral_embedded_checkout', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'viral_embedded_checkout');
});

test.describe('Viral Embedded Checkout Generator and Loop', () => {
  test('should allow generating checkout embed and verify branding loop', async ({ page, loginAs, adminUser }) => {
    // 1. Navigate to dashboard
    await loginAs(page, adminUser);
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    // 2. We mock localStorage if needed, but fixtures set it.
    await page.evaluate(() => { localStorage.setItem('has_pro', 'true'); window.dispatchEvent(new Event('storage')); });

    // 3. Navigate to the generator page directly
    await page.goto('/embed-checkout-generator');

    // Verify page content
    await expect(page.getByRole('heading', { name: 'Checkout Widget' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Configure' })).toBeVisible();

    // Fill out the configuration
    const productInput = page.getByPlaceholder('e.g. Premium Cake');
    await productInput.fill('Viral Pro Course');

    const priceInput = page.getByPlaceholder('e.g. 45.00');
    await priceInput.fill('199.99');

    // Verify live preview updates
    await expect(page.getByRole('heading', { name: 'Viral Pro Course' })).toBeVisible();
    await expect(page.getByText('$199.99')).toBeVisible();

    // Click to generate code
    await page.getByRole('button', { name: 'Get Widget Code' }).click();

    // The modal opens and shows textarea with iframe
    const textarea = page.locator('textarea[readonly]');
    await expect(textarea).toBeVisible();
    const embedCode = await textarea.inputValue();
    expect(embedCode).toContain('<iframe');
    expect(embedCode).toContain('/embed/checkout?');
    expect(embedCode).toContain('Viral%20Pro%20Course');

    // Close modal
    await page.getByRole('button', { name: 'Close' }).click();

    // 4. Navigate to the generated iframe URL
    // Extract the URL from the iframe src
    const srcMatch = embedCode.match(/src="([^"]+)"/);
    expect(srcMatch).not.toBeNull();
    const generatedUrl = srcMatch![1];

    // Go to the iframe URL directly
    await page.goto(generatedUrl);

    // Verify the embed view
    await expect(page.getByRole('heading', { name: 'Viral Pro Course' })).toBeVisible();
    await expect(page.getByText('$199.99')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Buy Now' })).toBeVisible();

    // Verify the "Powered by OHC" footer
    const poweredByLink = page.getByRole('link', { name: /Powered by OHC/i });
    await expect(poweredByLink).toBeVisible();

    // Verify where it points to ensure the viral loop is intact
    const href = await poweredByLink.getAttribute('href');
    expect(href).toContain('/onboarding?ref=');
    expect(href).toContain('source=checkout_embed');

    // Test the Buy Now action initiates checkout
    await page.getByRole('button', { name: 'Buy Now' }).click();
    await expect(page.getByText('Checkout process initiated.')).toBeVisible();
  });
});
