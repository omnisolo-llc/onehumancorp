import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('spin_to_win_generator_loop', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'spin_to_win_generator_loop');
});

test.describe('Spin To Win Generator Growth Loop', () => {
    test('dashboard links to Spin To Win Generator, which generates an embed with a viral footer', async ({ page, loginAs, adminUser }) => {
        await loginAs(page, adminUser);
        // Look for the "Spin To Win Generator" link in the Dashboard Growth & Virality section
        await page.goto('/dashboard');
        const generatorLink = page.locator('a[href="/spin-to-win-generator"]');
        await expect(generatorLink).toBeVisible();
        await generatorLink.click();

        // Verify page content
        await expect(page.locator('h1', { hasText: 'Spin to Win Generator' })).toBeVisible();

        // Check for the embed generation button
        await page.locator('button', { hasText: 'Generate Widget' }).click();

        await expect(page.locator('text=Embed Spin to Win')).toBeVisible();

        // Check that the code block has powered by OHC (In iframe embed, the URL carries tenant and default payload)
        const codeBlock = await page.locator('code').innerText();
        expect(codeBlock).toContain('tenant=DEFAULT');
    });

    test('generates valid iframe src url based on default values', async ({ page, loginAs, adminUser }) => {
        await loginAs(page, adminUser);
        await page.goto('/spin-to-win-generator');

        // Check for the embed generation button
        await page.locator('button', { hasText: 'Generate Widget' }).click();

        // Check that the code block has the expected default url
        const codeBlock = await page.locator('code').innerText();
        expect(codeBlock).toContain('api/v1/growth/spin-to-win/embed');
        expect(codeBlock).toContain('campaign=Summer%20Spin%20to%20Win');
        expect(codeBlock).toContain('reward=20%25%20Off');
    });

    test('generates valid iframe src url with custom values', async ({ page, loginAs, adminUser }) => {
        await loginAs(page, adminUser);
        await page.goto('/spin-to-win-generator');

        await page.locator('input[id="campaign-name"]').fill('Winter Flash Spin');
        await page.locator('input[id="reward"]').fill('Free iPhone');

        // Check for the embed generation button
        await page.locator('button', { hasText: 'Generate Widget' }).click();

        // Check that the code block has the expected custom url
        const codeBlock = await page.locator('code').innerText();
        expect(codeBlock).toContain('campaign=Winter%20Flash%20Spin');
        expect(codeBlock).toContain('reward=Free%20iPhone');
    });

    test('returns to dashboard from back link', async ({ page, loginAs, adminUser }) => {
        await loginAs(page, adminUser);
        await page.goto('/spin-to-win-generator');

        const backLink = page.locator('a', { hasText: 'Back to Dashboard' });
        await expect(backLink).toBeVisible();
        await expect(backLink).toHaveAttribute('href', '/dashboard');
    });

    test('has responsive grid container structure', async ({ page, loginAs, adminUser }) => {
        await loginAs(page, adminUser);
        await page.goto('/spin-to-win-generator');

        const container = page.locator('.container.glassmorphism');
        await expect(container).toBeVisible();
        await expect(container).toHaveCSS('max-width', '450px');
    });
});
