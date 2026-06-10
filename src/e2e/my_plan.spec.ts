import { test, expect } from './fixtures';

test.describe('My Plan Dashboard', () => {

  test('should display the My Plan header', async ({ page }) => {
<<<<<<< HEAD
    await page.goto('/cost-dashboard.html');
=======
    await page.goto('/plan');
>>>>>>> 359e384d (feat(memory): Implement AgentMemoryService for tenant-isolated episodic memory)
    await expect(page.locator('h1', { hasText: 'My Plan' })).toBeVisible({ timeout: 10000 });
  });

  test('should display current plan details', async ({ page }) => {
<<<<<<< HEAD
    await page.goto('/cost-dashboard.html');
    await expect(page.locator('#my-plan-name')).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Plan:' })).toBeVisible();
  });

  test('should display estimated next bill', async ({ page }) => {
    await page.goto('/cost-dashboard.html');
    await expect(page.locator('#my-plan-next-bill')).toBeVisible();
    await expect(page.locator('div.stat-title', { hasText: 'Estimated Next Bill' })).toBeVisible();
  });

  test('should display AI Actions usage', async ({ page }) => {
    await page.goto('/cost-dashboard.html');
    await expect(page.locator('div.stat-title', { hasText: 'AI actions used this month' })).toBeVisible();
  });

  test('should display Storage usage', async ({ page }) => {
    await page.goto('/cost-dashboard.html');
    await expect(page.locator('div.stat-title', { hasText: 'Storage used' })).toBeVisible();
=======
    await page.goto('/plan');
    await expect(page.locator('#my-plan-name')).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Plan:' })).toBeVisible();
    await expect(page.locator('span', { hasText: 'Active' })).toBeVisible();
  });

  test('should display estimated next bill', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.locator('#my-plan-next-bill')).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Estimated Next Bill:' })).toBeVisible();
  });

  test('should display AI Actions usage', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.locator('h2', { hasText: 'Your Current Usage' })).toBeVisible();
    await expect(page.locator('span', { hasText: 'AI actions used this month' })).toBeVisible();
  });

  test('should display Storage usage', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.locator('span', { hasText: 'Storage used' })).toBeVisible();
  });

  test('should display management action buttons', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.locator('h3', { hasText: 'View Cost Details' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Change Plan' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Download Invoice' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Cancel Subscription' })).toBeVisible();
>>>>>>> 359e384d (feat(memory): Implement AgentMemoryService for tenant-isolated episodic memory)
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

});
