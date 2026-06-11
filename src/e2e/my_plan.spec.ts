import { test, expect } from './fixtures';

test.describe('My Plan Dashboard', () => {

  test('should display the My Plan header', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.locator('h1', { hasText: 'My Plan' })).toBeVisible({ timeout: 10000 });
  });

  test('should display current plan details', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.locator('#my-plan-name')).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Plan:' })).toBeVisible();
  });

  test('should display estimated next bill', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.locator('#my-plan-next-bill')).toBeVisible();
    await expect(page.locator('div.stat-title', { hasText: 'Estimated Next Bill' })).toBeVisible();
  });

  test('should display AI Actions usage', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.locator('div.stat-title', { hasText: 'AI actions used this month' })).toBeVisible();
  });

  test('should display Storage usage', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await expect(page.locator('div.stat-title', { hasText: 'Storage used' })).toBeVisible();
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

  test('should navigate to pricing page when Change Plan is clicked', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.locator('h1', { hasText: 'My Plan' })).toBeVisible({ timeout: 10000 });
    const changePlanButton = page.locator('button', { hasText: 'Change Plan' });
    await expect(changePlanButton).toBeVisible();
    await changePlanButton.click();
    await expect(page.url()).toContain('/pricing');
  });

  test('should show invoice message when Download Invoice is clicked', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.locator('h1', { hasText: 'My Plan' })).toBeVisible({ timeout: 10000 });
    const downloadButton = page.locator('button', { hasText: 'Download Invoice' });
    await expect(downloadButton).toBeVisible();
    await downloadButton.click();
    await expect(page.locator('text=Invoice download is ready for your current billing period.')).toBeVisible();
  });

  test('should mock cancel subscription and show success message', async ({ page, request }) => {
    // Intercept confirm dialog and accept it
    page.on('dialog', async dialog => {
      await dialog.accept();
    });

    await page.goto('/plan');
    await expect(page.locator('h1', { hasText: 'My Plan' })).toBeVisible({ timeout: 10000 });

    // We mock the fetch request for cancelling subscription to avoid modifying database state
    await page.route('/api/billing/cancel-subscription', async route => {
      await route.fulfill({ status: 200, json: { success: true } });
    });

    const cancelButton = page.locator('button', { hasText: 'Cancel Subscription' });
    await expect(cancelButton).toBeVisible();
    await cancelButton.click();

    await expect(page.locator('text=Subscription canceled successfully.')).toBeVisible();
  });
});
