import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('growth_time_savings_widget', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'growth_time_savings_widget');
});

test.describe('Growth Time Savings Widget UI', () => {
  test('should display AI savings widget on dashboard', async ({ page, loginAs, adminUser }) => {
    // Login
    await loginAs(page, adminUser);

    // Navigate to dashboard where the widget is embedded
    await page.goto('/dashboard.html');
    await page.waitForLoadState('networkidle');

    // Verify milestone banner exists
    await expect(page.getByRole('heading', { name: /You saved \d+ hours this week/i })).toBeVisible();

    // Verify tweet share button exists
    await expect(page.getByRole('button', { name: /Share to get 7 Days Pro/i })).toBeVisible();
  });
});
