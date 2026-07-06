import { test, expect } from './fixtures';

test.describe('Interactive ROI Calculator Generator', () => {
  test('should display builder, handle config changes, soft paywall, and code generation', async ({ page }) => {
    // Navigate to the generator page
    await page.goto('/ui/interactive-roi-calculator.html');

    // Verify title and page elements
    await expect(page).toHaveTitle(/Interactive ROI Calculator/);
    await expect(page.locator('h1')).toHaveText('Interactive ROI Calculator');

    // Verify default preview values
    await expect(page.locator('#preview-title')).toHaveText('OHC Pro Services ROI');
    await expect(page.locator('#preview-return')).toHaveText('$3,000');

    // Fill custom values
    await page.fill('#service-name', 'Awesome Marketing');
    await page.fill('#base-investment', '500');
    await page.fill('#roi-multiplier', '5');

    // Verify preview updates instantly
    await expect(page.locator('#preview-title')).toHaveText('Awesome Marketing ROI');
    // 500 * 5 = 2500
    await expect(page.locator('#preview-return')).toHaveText('$2,500');

    // Test branding soft paywall
    const removeBrandingCheckbox = page.locator('#remove-branding');

    // Clear pro status first
    await page.evaluate(() => { localStorage.setItem('has_pro', 'false'); window.dispatchEvent(new Event('storage')); });

    // Try to remove branding, modal should appear
    await removeBrandingCheckbox.locator('..').click();
    await expect(page.locator('#paywall-modal')).toHaveCSS('display', 'flex');

    // Upgrade to Pro in modal
    await page.click('#upgrade-btn');

    // Modal should disappear, toggle should be checked, and preview branding should hide
    await expect(page.locator('#paywall-modal')).not.toHaveCSS('display', 'flex');
    await expect(removeBrandingCheckbox).toBeChecked();
    await expect(page.locator('#preview-footer')).not.toBeVisible();

    // Click Generate Embed Code
    const generateBtn = page.locator('#generate-btn');
    await generateBtn.click();

    // Result area should appear
    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    // Check embed code contents
    const embedCode = await page.inputValue('#embed-code');
    expect(embedCode).toContain('Awesome%20Marketing');
    expect(embedCode).toContain('inv=500');
    expect(embedCode).toContain('mult=5');
    expect(embedCode).toContain('branding=false');
  });

  test('embed widget should correctly calculate ROI and display branding viral loop', async ({ page, context }) => {
    // We navigate to the embed directly to simulate being within an iframe
    await page.goto('/ui/roi-embed.html?tenant=e2e-tenant&service=Test%20Service&inv=1000&mult=4&branding=true');

    // Initial state check
    await expect(page.locator('#service-title')).toHaveText('Test Service ROI');
    await expect(page.locator('#investment-display')).toHaveText('$1,000');
    await expect(page.locator('#roi-result')).toHaveText('$4,000');

    // Interact with slider
    const slider = page.locator('#investment-slider');
    await slider.fill('2000');
    // Using evaluate to trigger the input event properly for range sliders
    await slider.evaluate((node) => {
        node.dispatchEvent(new Event('input', { bubbles: true }));
    });

    // Check new calculation (2000 * 4 = 8000)
    await expect(page.locator('#investment-display')).toHaveText('$2,000');
    await expect(page.locator('#roi-result')).toHaveText('$8,000');

    // Check viral loop branding footer
    const brandingLink = page.locator('#branding-link');
    await expect(brandingLink).toBeVisible();
    await expect(brandingLink).toHaveText('⚡ Powered by OHC');

    // Check the link URL contains the correct referral parameters
    const href = await brandingLink.getAttribute('href');
    expect(href).toContain('ref=e2e-tenant');
    expect(href).toContain('source=roi_calculator_embed');
  });
});
