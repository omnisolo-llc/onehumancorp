import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('growth_referral_widget_milestone', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'growth_referral_widget_milestone');
});

test.describe('Growth Referral Widget Milestone UI', () => {
  test('keeps order milestones unavailable without a verified business count', async ({ page, loginAs, unlimitedAdminUser }) => {
    // Login
    await loginAs(page, unlimitedAdminUser);

    // Navigate to /team where the widget is embedded
    await page.goto('/team');
    await page.waitForLoadState('networkidle');

    await expect(page.getByRole('heading', { name: 'Order milestones unavailable' })).toBeVisible();
    await expect(page.getByText('A verified order count for this business is required before a milestone can be displayed or shared.')).toBeVisible();
    await expect(page.getByRole('link', { name: /Share to WhatsApp/i })).toHaveCount(0);
    await expect(page.getByRole('img', { name: '10th Order Milestone' })).toHaveCount(0);
    await expect(page.locator('[src*="milestone_id=10th_order"]')).toHaveCount(0);
  });
});
