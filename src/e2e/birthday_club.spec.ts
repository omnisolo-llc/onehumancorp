import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('birthday_club_loop', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'birthday_club_loop');
});

test.describe('Birthday Club Growth Loop', () => {
    test('dashboard links to Birthday Club Program, which generates an embed with a viral footer', async ({ page, request }) => {
        await page.goto('/birthday-club');

        // Verify page content
        await expect(page.locator('h1', { hasText: 'Birthday Club Builder' })).toBeVisible();

        // Set discount amount
        const discountInput = page.locator('input[type="number"]').first();
        await discountInput.fill('25');

        // Check for the embed generation button and text area
        await page.locator('button', { hasText: 'Generate Widget Embed' }).click();

        const modal = page.locator('.fixed.inset-0').first();
        await expect(modal).toBeVisible();

        // Ensure the referral growth loop is intact in the generated iframe code
        const embedCode = await page.locator('pre').innerText();
        expect(embedCode).toContain('<iframe src="https://ohc.app/api/v1/growth/birthday-club/embed');
        expect(embedCode).toContain('⚡ Powered by OHC');

        // Check the backend embed API endpoint directly to ensure it renders correctly
        const response = await request.get('/api/v1/growth/birthday-club/embed?tenant=maya-cakes&discount=25');
        expect(response.status()).toBe(200);

        const html = await response.text();
        expect(html).toContain('25% off');
        expect(html).toContain('⚡ Powered by OHC');

        // Check the backend embed API endpoint directly when branding is hidden
        const responseNoBranding = await request.get('/api/v1/growth/birthday-club/embed?tenant=maya-cakes&discount=25&hideBranding=true');
        expect(responseNoBranding.status()).toBe(200);

        const htmlNoBranding = await responseNoBranding.text();
        expect(htmlNoBranding).toContain('25% off');
        expect(htmlNoBranding).not.toContain('⚡ Powered by OHC');
    });

    test('should show soft paywall when attempting to remove branding without pro', async ({ page }) => {
        await page.goto('/birthday-club');
        await page.evaluate(() => {
            localStorage.setItem('tenant', 'e2e-test-store');
            localStorage.setItem('has_pro', 'false');
        });
        await page.reload();

        const toggle = page.locator('input[type="checkbox"]');
        await toggle.click({ force: true });

        // Soft paywall should appear
        await expect(page.locator('text=Pro Feature')).toBeVisible();
        await expect(page.getByRole('button', { name: 'Upgrade to Pro' }).first()).toBeVisible();

        await page.locator('#close-paywall').click();
    });

    test('should hide branding when pro is enabled and toggle is clicked', async ({ page }) => {
        await page.goto('/birthday-club');
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

        await page.locator('button', { hasText: 'Generate Widget Embed' }).click();
        const modal = page.locator('.fixed.inset-0').first();
        await expect(modal).toBeVisible();

        // The textarea code should NOT include the branding
        const preCode = await page.locator('pre').innerText();
        expect(preCode).not.toContain('>⚡ Powered by OHC</a></div>`');
        expect(preCode).toContain('hideBranding=true');
    });

    test('submitting the birthday club form calls the capture endpoint', async ({ page, request }) => {
        await page.goto('/birthday-club');

        // Find the iframe
        const frame = page.frameLocator('iframe');

        // Fill the form inside the iframe
        await frame.locator('input[id="name"]').fill('Test User');
        await frame.locator('input[id="email"]').fill('test@example.com');
        await frame.locator('input[id="birthday"]').fill('1990-01-01');

        // Intercept the API call
        const capturePromise = page.waitForResponse(response =>
            response.url().includes('/api/v1/growth/birthday-club/capture') && response.status() === 200
        );

        // Handle the alert
        page.on('dialog', dialog => dialog.accept());

        // Click join
        await frame.locator('button', { hasText: 'Join the Club' }).click();

        // Wait for the capture request to complete
        await capturePromise;
    });
});
