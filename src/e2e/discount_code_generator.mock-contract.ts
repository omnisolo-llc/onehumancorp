import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('discount_code_generator_loop', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'discount_code_generator_loop');
});

test.describe('Discount Code Generator Growth Loop', () => {
    test('dashboard links to Discount Code Generator, which generates an embed with a viral footer', async ({ page, request }) => {
        // Look for the "Discount Code Generator" link in the Dashboard Growth & Virality section
        await page.goto('/dashboard');
        const generatorLink = page.locator('a[href="/discount-code-generator"]');
        await expect(generatorLink).toBeVisible();
        await generatorLink.click();

        // Verify page content
        await expect(page.locator('h1', { hasText: 'Discount Code Generator' })).toBeVisible();

        // Set discount and code
        const discountInput = page.locator('input[placeholder="e.g. 20% or $10"]');
        const codeInput = page.locator('input[placeholder="e.g. SUMMER20"]');
        await discountInput.fill('50%');
        await codeInput.fill('HALFOFF');

        // Check for the embed generation button and text area
        await page.locator('button', { hasText: 'Generate Widget Embed' }).click();

        const modal = page.locator('.fixed.inset-0').first();
        await expect(modal).toBeVisible();

        // Ensure the referral growth loop is intact in the generated iframe code
        const embedCode = await page.locator('pre').innerText();
        expect(embedCode).toContain('<iframe src="https://ohc.app/api/v1/growth/discount-code/embed');
        expect(embedCode).toContain('⚡ Powered by OHC');

        // Check the backend embed API endpoint directly to ensure it renders correctly
        const response = await request.get('/api/v1/growth/discount-code/embed?tenant=test-tenant&discount=50%25&code=HALFOFF');
        expect(response.status()).toBe(200);

        const html = await response.text();
        expect(html).toContain('50% OFF');
        expect(html).toContain('HALFOFF');
        expect(html).toContain('⚡ Powered by OHC');
    });

    test('should show soft paywall when attempting to remove branding without pro', async ({ page }) => {
        await page.goto('/discount-code-generator');
        await page.evaluate(() => {
            localStorage.setItem('tenant', 'e2e-test-store');
            localStorage.setItem('has_pro', 'false');
        });
        await page.reload();

        const toggle = page.locator('input[type="checkbox"]');
        await toggle.click({ force: true }); // It's hidden behind styling

        // Soft paywall should appear
        await expect(page.locator('text=Pro Feature')).toBeVisible();
        await expect(page.locator('text=Upgrade to Pro')).toBeVisible();
    });

    test('should hide branding when pro is enabled and toggle is clicked', async ({ page }) => {
        await page.goto('/discount-code-generator');
        await page.evaluate(() => {
            localStorage.setItem('tenant', 'e2e-test-store');
            localStorage.setItem('has_pro', 'true');
        });
        await page.reload();

        const toggle = page.locator('input[type="checkbox"]');
        await toggle.click({ force: true });

        // Soft paywall should not appear
        await expect(page.locator('text=Pro Feature')).not.toBeVisible();

        // Preview section should hide the branding
        await expect(page.locator('text=⚡ Powered by OHC')).not.toBeVisible();

        // Need to fill inputs first to enable button
        const discountInput = page.locator('input[placeholder="e.g. 20% or $10"]');
        const codeInput = page.locator('input[placeholder="e.g. SUMMER20"]');
        await discountInput.fill('50%');
        await codeInput.fill('HALFOFF');

        await page.locator('button', { hasText: 'Generate Widget Embed' }).click();
        const modal = page.locator('.fixed.inset-0').first();
        await expect(modal).toBeVisible();

        // The textarea code should NOT include the branding
        const preCode = await page.locator('pre').innerText();
        expect(preCode).not.toContain('>⚡ Powered by OHC</a></div>`'); // The static html part
        expect(preCode).toContain('hideBranding=true');
    });
});
