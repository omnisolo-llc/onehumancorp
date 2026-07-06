import { test, expect } from './fixtures';

test.describe('Documentation & Help Features', () => {

  test('should display tooltips correctly', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');

    const tooltipTarget = page.locator('#dashboard-tooltip');
    await tooltipTarget.waitFor({ state: "visible", timeout: 10000 });
    await tooltipTarget.hover();
    await expect(page.locator('[role="tooltip"]')).toBeVisible({ timeout: 5000 });
  });

  test('should open help widget and view articles', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/');

    // Help widget button
    const helpBtn = page.getByRole('button', { name: 'Help', exact: true });
    await expect(helpBtn).toBeVisible();
    await helpBtn.click();

    // Verify widget opened by checking if "Help Center" title inside the widget is visible
    const helpCenterTitle = page.getByRole('heading', { name: 'Help Center', exact: true });
    await expect(helpCenterTitle).toBeVisible();

    // Check for article (since backend might be returning it slowly, add retries or let playwright handle it)
    await expect(page.getByText('Getting Started')).toBeVisible({ timeout: 10000 });
  });

  test('should search for an article', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/help');

    const searchInput = page.getByPlaceholder('Search for help articles and videos...');
    await searchInput.fill('payment');

    // Wait for the results to update
    await expect(page.getByText('Accepting Payments')).toBeVisible({ timeout: 10000 });
  });

  test('should show video tutorials', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/help/videos');

    await expect(page.getByRole('heading', { name: 'Video Guides', exact: true })).toBeVisible();
    await expect(page.getByText('How to set up your first store easily')).toBeVisible({ timeout: 10000 });
  });

  test('should display inventory tooltip correctly', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');
    const tooltipTarget = page.locator('[id="inventory-tooltip"]');
    await tooltipTarget.waitFor({ state: "visible", timeout: 10000 });
    await tooltipTarget.hover();
    await expect(page.locator('[role="tooltip"]')).toBeVisible({ timeout: 5000 });
  });

  test('should display orders tooltip correctly', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');
    const tooltipTarget = page.locator('[id="orders-tooltip"]');
    await tooltipTarget.waitFor({ state: "visible", timeout: 10000 });
    await tooltipTarget.hover();
    await expect(page.locator('[role="tooltip"]')).toBeVisible({ timeout: 5000 });
  });

  test('should display total sales tooltip correctly', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');
    const tooltipTarget = page.locator('[id="total-sales-tooltip"]');
    await tooltipTarget.waitFor({ state: "visible", timeout: 10000 });
    await tooltipTarget.hover();
    await expect(page.locator('[role="tooltip"]')).toBeVisible({ timeout: 5000 });
  });

  test('should display recent orders tooltip correctly', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');
    const tooltipTarget = page.locator('[id="recent-orders-tooltip"]');
    await tooltipTarget.waitFor({ state: "visible", timeout: 10000 });
    await tooltipTarget.hover();
    await expect(page.locator('[role="tooltip"]')).toBeVisible({ timeout: 5000 });
  });

  test('should display inbox activity tooltip correctly', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');
    const tooltipTarget = page.locator('[id="inbox-activity-tooltip"]');
    await tooltipTarget.waitFor({ state: "visible", timeout: 10000 });
    await tooltipTarget.hover();
    await expect(page.locator('[role="tooltip"]')).toBeVisible({ timeout: 5000 });
  });

});
