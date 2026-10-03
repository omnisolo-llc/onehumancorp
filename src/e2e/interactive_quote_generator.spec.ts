import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';
import { requireLoopbackUrl } from './support/recorded_invitation';

test.beforeEach(({ baseURL }) => requireLoopbackUrl(baseURL ?? ''));

test('interactive_quote_generator_loop', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'interactive_quote_generator_loop');
});

test.describe('Interactive Quote Generator Growth Loop', () => {
    test('dashboard links to Interactive Quote Generator, which generates an embed with a viral footer', async ({ page, context, loginAs, adminUser }) => {
        await loginAs(page, adminUser);
        // Look for the "Interactive Quote Generator" link in the Dashboard Growth & Virality section
        await page.goto('/dashboard');
        const generatorLink = page.locator('a[href="/interactive-quote-generator"]');
        await expect(generatorLink).toBeVisible();
        await generatorLink.click();

        // Verify page content
        await expect(page.locator('h1', { hasText: 'Interactive Quote Generator' }).first()).toBeVisible();

        requireLoopbackUrl(page.url());
        await expect(page.getByRole('button', { name: 'Copy Embed Code', exact: true })).toBeEnabled();
        await expect(page.getByRole('textbox', { name: 'Verified quote embed code' })).toHaveAttribute('aria-busy', 'false');
        // Exercise the browser clipboard with explicit permissions for this test origin.
        await context.grantPermissions(['clipboard-read', 'clipboard-write'], { origin: new URL(page.url()).origin });
        const embedCode = await page.locator('textarea[readonly]').inputValue();
        const source = await page.evaluate(code => new DOMParser().parseFromString(code, 'text/html').querySelector('iframe')?.getAttribute('src'), embedCode);
        expect(source).toBeTruthy();
        const calculator = new URL(source!);
        expect(calculator.origin).toBe(new URL(page.url()).origin);
        expect(calculator.pathname).toBe('/quote-calculator');
        expect(calculator.searchParams.get('tenant')).toBe(adminUser.organizationId);
        await page.bringToFront();
        await expect.poll(() => page.evaluate(() => document.hasFocus())).toBe(true);
        await page.locator('button', { hasText: 'Copy Embed Code' }).click();

        await expect(page.getByRole('button', { name: 'Code Copied!', exact: true })).toBeVisible();
        await expect.poll(() => page.evaluate(() => navigator.clipboard.readText())).toBe(embedCode);

        // Now test the quote calculator endpoint
        await page.goto(calculator.toString());

        await expect(page.locator('h1', { hasText: 'Custom Cake Design Quote' })).toBeVisible();
        await expect(page.locator('text=Base Price:')).toBeVisible();

        // Wait and find the total
        const slider = page.locator('input[type="range"]');
        await slider.fill('20');

        // Let's verify total calculation (50 + 20 * 5) = 150
        await expect(page.locator('text=$150')).toBeVisible();

        // Check the footer viral link
        await expect(page.locator('a', { hasText: /OmniSolo/ })).toBeVisible();
    });
});
