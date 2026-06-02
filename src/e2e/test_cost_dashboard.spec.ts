import { test, expect } from './fixtures';

test.describe('Cost Dashboard UI test', () => {
  test('Owner navigates to cost dashboard from my plan', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // The button might just be labelled "Billing" on the side bar, or there could be a route we can hit.
    // Let's directly navigate to the screens since they are embedded UI screens.
    // However, playwright needs a full page reload or we can click if the nav exists.
    // In currentAppSmoke, it goes directly to URLs. Let's do that for reliability in embedded app.

    // Evaluate javascript to change screen
    await page.evaluate(() => {
        // @ts-ignore
        window.showScreen('my-plan-screen');
    });

    // Check elements dynamically populated on My Plan screen
    await expect(page.locator('#my-plan-screen')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();
    await expect(page.locator('#my-plan-name')).toContainText('Free');

    // Click view cost details (this uses showScreen in embedded UI)
    await page.getByRole('button', { name: 'View Cost Details' }).click();

    // Verify Cost Dashboard Screen
    await expect(page.locator('#cost-dashboard-screen')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Cost Transparency' })).toBeVisible();

    // Verify dynamic metrics are present
    await expect(page.locator('#cost-dashboard-period')).toBeVisible();
    await expect(page.locator('#cost-dashboard-total')).toBeVisible();
    await expect(page.locator('#cost-dashboard-revenue')).toBeVisible();
    await expect(page.locator('#cost-dashboard-llm')).toBeVisible();
    await expect(page.locator('#cost-dashboard-storage')).toBeVisible();
    await expect(page.locator('#cost-dashboard-payment-fees')).toBeVisible();
  });
});
