import { test, expect } from './fixtures';

test.describe('My Plan Dashboard', () => {

  test('should display the My Plan header', async ({ page }) => {
    await page.goto('/cost-dashboard.html');
    await expect(page.locator('h1', { hasText: 'My Plan' })).toBeVisible({ timeout: 10000 });
    await expect(page.locator('h2', { hasText: 'Cost Transparency Dashboard' })).toBeVisible({ timeout: 10000 });
  });

  test('should display current plan details', async ({ page }) => {
    await page.goto('/cost-dashboard.html');
    await expect(page.locator('h2', { hasText: 'Plan:' })).toBeVisible();
  });

  test('should display estimated next bill', async ({ page }) => {
    await page.goto('/cost-dashboard.html');
    await expect(page.locator('.stat-title', { hasText: 'Estimated Next Bill' })).toBeVisible();
  });

  test('should display AI Actions usage', async ({ page }) => {
    await page.goto('/cost-dashboard.html');
    await expect(page.locator('.stat-title', { hasText: 'AI actions used this month' })).toBeVisible();
  });

  test('should display Storage usage', async ({ page }) => {
    await page.goto('/cost-dashboard.html');
    await expect(page.locator('.stat-title', { hasText: 'Storage used' })).toBeVisible();
  });

  test('should return correct JSON payload from backend API', async ({ request }) => {
    const response = await request.get('/api/billing/my-plan');
    expect(response.ok()).toBeTruthy();
    const data = await response.json();

    expect(data).toHaveProperty('current_plan');
    expect(data).toHaveProperty('ai_actions_used');
    expect(data).toHaveProperty('storage_used_bytes');
    expect(data).toHaveProperty('next_bill_estimated');
  });

  test('should navigate to pricing page when Upgrade is clicked', async ({ page }) => {
    await page.goto('/cost-dashboard.html');
    const changePlanButton = page.locator('button', { hasText: 'Upgrade Plan' });
    await expect(changePlanButton).toBeVisible();
    await changePlanButton.click();
    await expect(page.url()).toContain('/pricing');
  });

  test('should download invoice when Download Invoice is clicked', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/cost-dashboard.html');
    await expect(page.locator('h1', { hasText: 'My Plan' })).toBeVisible({ timeout: 10000 });
    const downloadButton = page.locator('button', { hasText: 'Download Invoice' });
    await expect(downloadButton).toBeVisible();
    await downloadButton.click();
    await expect(page.locator('text=Invoice download is ready for your current billing period.')).toBeVisible();
  });

  test('should cancel subscription and show success message', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);

    // Intercept confirm dialog and accept it
    page.on('dialog', async dialog => {
      await dialog.accept();
    });

    await page.goto('/cost-dashboard.html');
    await expect(page.locator('h1', { hasText: 'My Plan' })).toBeVisible({ timeout: 10000 });

    const cancelButton = page.locator('button', { hasText: 'Cancel Subscription' });
    await expect(cancelButton).toBeVisible();
    await cancelButton.click();

    await expect(page.locator('text=Subscription canceled successfully.')).toBeVisible();
  });

});
