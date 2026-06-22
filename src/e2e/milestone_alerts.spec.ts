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
});
