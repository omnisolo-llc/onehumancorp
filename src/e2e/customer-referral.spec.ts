import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test customer_referral_loop', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'customer_referral_loop');
});

test.describe('Customer Referral Program Growth Loop', () => {
    test('dashboard links to Customer Referral Program, which generates an embed with a viral footer', async ({ page, request }) => {
        // Look for the "Customer Referral Program" link in the Dashboard Growth & Virality section
        await page.goto('/dashboard');
        const referralLink = page.locator('a[href="/customer-referral-program"]');
        await expect(referralLink).toBeVisible();
        await referralLink.click();

        // Verify page content
        await expect(page.locator('h1', { hasText: 'Customer Referral Program' })).toBeVisible();
        await expect(page.locator('text=Turn your customers into advocates')).toBeVisible();

        // Set give and get amounts
        const giveInput = page.locator('input[type="number"]').first();
        const getInput = page.locator('input[type="number"]').nth(1);
        await giveInput.fill('20');
        await getInput.fill('25');

        // Verify the preview updates
        await expect(page.locator('h3:has-text("Give $20, Get $25")')).toBeVisible();
        await expect(page.locator('h4:has-text("Give $20, Get $25")')).toBeVisible();

        // Check for the embed generation button and text area
        await page.locator('button', { hasText: 'Generate Widget Embed' }).click();

        const modal = page.locator('.fixed.inset-0').first();
        await expect(modal).toBeVisible();

        // Ensure the referral growth loop is intact in the generated iframe code
        const embedCode = await page.locator('pre').innerText();
        expect(embedCode).toContain('<iframe src="https://ohc.app/api/v1/growth/customer-referral/embed');
        expect(embedCode).toContain('⚡ Powered by OHC');

        // Check the backend embed API endpoint directly to ensure it renders correctly
        const response = await request.get('/api/v1/growth/customer-referral/embed?tenant=maya-cakes&give=20&get=25');
        expect(response.status()).toBe(200);

        const html = await response.text();
        expect(html).toContain('Give $20, Get $25');
    });
});
