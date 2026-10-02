import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';
import { createRecordedOrderOwner, captureRecordedOrders } from './support/recorded_order_fixture';

test('growth_referral_widget_milestone', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'growth_referral_widget_milestone');
});

test.describe('Growth Referral Widget Milestone UI', () => {
  test('keeps an actual empty business free of invented order milestones', async ({ page, baseURL }) => {
    const owner = await createRecordedOrderOwner(page, baseURL, 0);
    const reading = captureRecordedOrders(page, owner);
    // Navigate to /team where the widget is embedded
    await page.goto('/team');
    await reading;

    await expect(page.getByText('No recorded orders yet.', { exact: true })).toBeVisible();
    await expect(page.getByRole('link', { name: /Share to WhatsApp/i })).toHaveCount(0);
    await expect(page.getByRole('img', { name: '10th Order Milestone' })).toHaveCount(0);
    await expect(page.locator('[src*="milestone_id=10th_order"]')).toHaveCount(0);
  });
});
