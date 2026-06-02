import { test, expect } from './fixtures';

test.describe('CUJ: Services Billing My Plan', () => {
  test('Owner can navigate to My Plan and view limits', async ({ page }) => {
    // Navigate from root
    await page.goto('/');

    const dashLink = page.getByRole('link', { name: 'Go to Dashboard' });
    if (await dashLink.isVisible()) {
      await dashLink.click();
    } else {
      const loginBtn = page.getByRole('button', { name: 'Log In' });
      if (await loginBtn.isVisible()) {
        await loginBtn.click();
      }
    }

    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // From dashboard, go to billing
    await page.getByRole('button', { name: 'Billing', exact: true }).click();

    // Verify header
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();

    // Verify Status Snapshot
    await expect(page.locator('#my-plan-name')).toContainText('Plan:');
    await expect(page.locator('#my-plan-next-bill')).toContainText('Estimated Next Bill:');

    // Verify Usage Section
    await expect(page.getByRole('heading', { name: 'Your Current Usage' })).toBeVisible();
    await expect(page.getByText('AI Actions Used')).toBeVisible();
    await expect(page.getByText('Storage Used')).toBeVisible();

    // Verify Management Actions
    await expect(page.getByRole('button', { name: 'View Cost Details' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Change Plan' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Download Invoice' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Cancel Subscription' })).toBeVisible();
  });
});
