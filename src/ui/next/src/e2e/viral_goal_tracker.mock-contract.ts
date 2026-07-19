import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('viral_goal_tracker_smoke', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'viral_goal_tracker_smoke');
});

test.describe('Viral Goal Tracker Widget', () => {
    test('generator page renders correctly and embed code contains branding by default', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);

        // Wait for dashboard to load then click link
        await page.goto('/dashboard.html');
        const link = page.locator('a[id="goal-tracker-btn"]');
        await expect(link).toBeVisible();
        await link.click();

        // Check the page header
        await expect(page.locator('h1', { hasText: 'Goal Tracker Builder' })).toBeVisible();

        // Configure the widget
        const targetInput = page.locator('input[type="number"]').first();
        await targetInput.fill('25');

        const rewardInput = page.locator('input[type="text"]').first();
        await rewardInput.fill('Awesome E2E Reward');

        // The preview section should load and display the branding
        await expect(page.getByText('Unlock: Awesome E2E Reward')).toBeVisible();
        await expect(page.getByText('25 target')).toBeVisible();
        await expect(page.locator('a', { hasText: '⚡ Powered by OHC' })).toBeVisible();

        // Check the generated embed code
        const embedCode = await page.locator('pre').first().innerText();
        expect(embedCode).toContain('target=25');
        expect(embedCode).toContain('reward=Awesome%20E2E%20Reward');
        expect(embedCode).toContain('hideBranding=false');
    });

    test('should show soft paywall when attempting to remove branding without pro', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);
        await page.goto('/dashboard.html');
        await page.evaluate(() => {
            localStorage.setItem('tenant', 'e2e-test-store');
            localStorage.setItem('has_pro', 'false');
        });
        await page.reload();

        const link = page.locator('a[id="goal-tracker-btn"]');
        await expect(link).toBeVisible();
        await link.click();

        const removeBrandingCheckbox = page.locator('#removeBranding');
        await removeBrandingCheckbox.check();

        // Soft paywall should appear
        const paywallModal = page.locator('h2', { hasText: 'Upgrade to Remove Branding' });
        await expect(paywallModal).toBeVisible();
    });

    test('should hide branding when pro is enabled and toggle is clicked', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);
        await page.goto('/dashboard.html');
        await page.evaluate(() => {
            localStorage.setItem('tenant', 'e2e-test-store');
            localStorage.setItem('has_pro', 'true');
        });
        await page.reload();

        const link = page.locator('a[id="goal-tracker-btn"]');
        await expect(link).toBeVisible();
        await link.click();

        const removeBrandingCheckbox = page.locator('#removeBranding');
        await removeBrandingCheckbox.check();

        // Soft paywall should not appear
        const paywallModal = page.locator('h2', { hasText: 'Upgrade to Remove Branding' });
        await expect(paywallModal).not.toBeVisible();

        // The generated link should NOT include the branding parameter as true
        const embedCode = await page.locator('pre').first().innerText();
        expect(embedCode).toContain('hideBranding=true');

        // The preview branding should be hidden
        await expect(page.locator('a', { hasText: '⚡ Powered by OHC' })).not.toBeVisible();
    });
});
