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

        // Check that the code block has powered by OHC
        const codeBlock = await page.locator('code').innerText();
        expect(codeBlock).toContain('Powered by OHC');
    });
});
