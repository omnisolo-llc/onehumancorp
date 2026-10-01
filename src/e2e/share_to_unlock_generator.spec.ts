import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('share_to_unlock_generator_loop', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'share_to_unlock_generator_loop');
});

test.describe('Share To Unlock Generator Growth Loop', () => {
    test('dashboard links to Share To Unlock Generator, which generates a viral share page', async ({ page, loginAs, adminUser }) => {
        await loginAs(page, adminUser);
        // Look for the "Share To Unlock Generator" link in the Dashboard Growth & Virality section
        await page.goto('/dashboard');
        const generatorLink = page.locator('a[href="/share-to-unlock-generator"]');
        await expect(generatorLink).toBeVisible();
        await generatorLink.click();

        // Verify page content
        await expect(page.locator('h1', { hasText: 'Share-to-Unlock Generator' })).toBeVisible();

        // Set up the campaign
        await page.locator('input[id="campaign-title"]').fill('Special Deal');
        await page.locator('input[id="reward"]').fill('Free Shipping');
        await page.locator('input[id="hidden-code"]').fill('FREESHIP');

        // Read the actual platform result and navigate to the configured preview.
        await page.context().grantPermissions(['clipboard-read', 'clipboard-write']);
        await page.locator('button', { hasText: 'Copy Link' }).click();
        await expect(page.locator('text=Copied!')).toBeVisible();

        const copied = await page.evaluate(() => navigator.clipboard.readText());
        const preview = new URL(copied);
        expect(preview.origin).toBe(new URL(page.url()).origin);
        expect(preview.pathname).toBe('/unlock');
        expect(preview.searchParams.get('title')).toBe('Special Deal');
        expect(preview.searchParams.get('reward')).toBe('Free Shipping');
        expect(preview.searchParams.get('code')).toBe('FREESHIP');
        expect(preview.searchParams.get('theme')).toBe('light');
        await page.goto(copied);
        await expect(page.getByText('Sharing does not verify reward eligibility')).toBeVisible();
        await expect(page.locator('#locked-badge')).toContainText('Locked');

        await expect(page.locator('h1', { hasText: 'Special Deal' })).toBeVisible();
        await expect(page.locator('text=Free Shipping')).toBeVisible();

        // Check the footer viral link
        await expect(page.locator('a', { hasText: '⚡ OmniSolo' })).toBeVisible();
    });
});
