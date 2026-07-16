import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('milestone_alerts_loop', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'milestone_alerts_loop');
});

test.describe('Milestone Alerts', () => {
    test('dashboard links to Milestone Alerts, which displays unlocked milestones and allows sharing', async ({ page, loginAs, adminUser }) => {
        await loginAs(page, adminUser);
        // Look for the "Milestone Alerts" link in the Dashboard Growth & Virality section
        await page.goto('/dashboard');
        const generatorLink = page.locator('a[href="/milestone-alerts"]');
        await expect(generatorLink).toBeVisible();
        await generatorLink.click();

        // Verify page content
        await expect(page.locator('h1', { hasText: 'Success Milestones 🏆' })).toBeVisible();

        // Click a milestone and test share options
        await expect(page.locator('text=First Sale!')).toBeVisible();

        await expect(page.locator('text=Invite a friend')).toBeVisible();
        await expect(page.locator('text=Download Achievement')).toBeVisible();
    });

    test('displays soft paywall when attempting to remove branding without pro', async ({ page, loginAs, adminUser }) => {
        // Authenticate using regular flow
        await loginAs(page, adminUser);

        // Setup local storage to disable Pro
        await page.evaluate(() => {
            window.localStorage.setItem('has_pro', 'false');
        });

        // Navigate to the milestone alerts page
        await page.goto('/milestone-alerts');

        await expect(page.getByRole('heading', { name: /Success Milestones/i })).toBeVisible();

        // Click a milestone
        await page.locator('.milestone-item').first().click();

        // Check for the remove branding checkbox
        const removeBrandingCheckbox = page.getByLabel(/Remove "Powered by OHC" Badge/i);
        await expect(removeBrandingCheckbox).toBeVisible();

        // Click it
        await removeBrandingCheckbox.click();

        // Soft paywall modal should appear
        await expect(page.getByText('Upgrade to Remove Branding')).toBeVisible();
        await expect(page.getByRole('button', { name: /Upgrade to Pro/i })).toBeVisible();
    });
});
