import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('growth_referral_widget_milestone');

test.describe('Growth Referral Widget Milestone UI', () => {
  test('should display 10th order milestone alert and card on Team Page', async ({ page, loginAs, unlimitedAdminUser }) => {
    // Login
    await loginAs(page, unlimitedAdminUser);

    // Navigate to /team where the widget is embedded
    await page.goto('/team');
    await page.waitForLoadState('networkidle');

    // Verify milestone banner exists
    await expect(page.getByRole('heading', { name: /10th Order! Share your success/i })).toBeVisible();

    // Verify WhatsApp share button exists
    await expect(page.getByRole('link', { name: /Share to WhatsApp/i })).toBeVisible();

    // Verify the milestone card image is loaded
    const milestoneImage = page.locator('img[alt="10th Order Milestone"]');
    await expect(milestoneImage).toBeVisible();
    await expect(milestoneImage).toHaveAttribute('src', /milestone_id=10th_order/);
  });
});
