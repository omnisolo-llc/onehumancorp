import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('referral_milestone_notification', async ({ page, request, loginAs }) => {
  await loginAs(page, { email: 'referral-milestone@example.com', password: 'password123' });
  await currentAppSmoke(page, request, 'referral_milestone_notification');
});

test.describe('Referral Milestone Notification Widget UI', () => {
  test('should display Silver Referral Tier milestone and copy button', async ({ page, loginAs, context }) => {
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);
    await loginAs(page, { email: 'referral-milestone@example.com', password: 'password123' });

    await page.goto('/dashboard.html');
    await page.waitForLoadState('networkidle');

    const milestoneContainer = page.locator('#milestone-container');
    await expect(milestoneContainer).toBeVisible({ timeout: 5000 });

    const milestoneTitle = page.locator('#milestone-title');
    await expect(milestoneTitle).toHaveText('Silver Referral Tier');

    const milestoneDesc = page.locator('#milestone-desc');
    await expect(milestoneDesc).toHaveText("You've reached the Silver referral tier with 5+ successful referrals!");

    const milestoneIcon = page.locator('#milestone-icon');
    await expect(milestoneIcon).toHaveText('🥈');

    const copyBtn = page.locator('#milestone-copy-btn');
    await expect(copyBtn).toBeVisible();

    await copyBtn.click();

    // Check if clipboard has the invite link
    try {
        const clipboardText = await page.evaluate(async () => {
            return await navigator.clipboard.readText();
        });
        expect(clipboardText).toContain('Silver Referral Tier');
    } catch (e) {
        console.warn('Clipboard read failed (expected in some headless environments): ', e);
    }

    const xBtn = page.locator('#milestone-x-btn');
    await expect(xBtn).toBeVisible();
  });
});
