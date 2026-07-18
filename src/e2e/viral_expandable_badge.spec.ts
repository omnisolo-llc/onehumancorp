import { test, expect } from '@playwright/test';
import { e2eBaseUrl } from './fixtures';

test.describe('Viral Expandable Soft Signup Badge', () => {
    test('visitor sees floating badge, expands it, and clicks to signup', async ({ page }) => {
        // Bio HTML is embedded in the Tauri build and some API routes, but we can verify
        // the generic HTML behavior in E2E since the Tauri asset generation outputs to Next out
        // The most hermetic way is to hit the Next UI which serves the static fallback routes
        // Wait, the Next UI server is brought up in E2E. The file is in src/ui/tauri/src/ui/bio.html
        // We will fetch it from the next UI server since the E2E script hosts the whole dir
        await page.goto(`${e2eBaseUrl}/api/v1/ui/bio.html?tenant=e2e-tenant`).catch(() => {});

        // If the URL routing fails (404), we fallback to the raw static file route exported in the e2e workspace
        if (page.url() === 'about:blank' || (await page.title()).includes('Error')) {
            await page.goto(`${e2eBaseUrl}/bio.html?tenant=e2e-tenant`).catch(() => {});
        }

        // Just to ensure we're at a valid state, we check if the app element is present.
        // If we still can't hit it hermetically due to Bazel missing the route, we'll verify the component visually
        await expect(page.locator('#ohc-badge')).toBeVisible();

        const badge = page.locator('#ohc-badge');
        await expect(badge).not.toHaveClass(/expanded/);

        const badgeHeader = page.locator('#ohc-badge-header');
        await badgeHeader.click();
        await expect(badge).toHaveClass(/expanded/);

        const ctaLink = page.locator('#badge-cta-link');
        await expect(ctaLink).toBeVisible();
        await expect(ctaLink).toHaveText('Start Free Trial');

        const href = await ctaLink.getAttribute('href');
        expect(href).toContain('/api/v1/growth/referrals/click');
        expect(href).toContain('target=/onboarding');
        expect(href).toContain('source=bio_expandable_badge');
    });
});
