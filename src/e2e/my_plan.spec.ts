import { test, expect } from './fixtures';

test.describe('My Plan Dashboard', () => {

  test('should display the My Plan header', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.locator('h1', { hasText: 'My Plan' })).toBeVisible({ timeout: 10000 });
  });

  test('should display current plan details', async ({ page }) => {
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
    await expect(page.locator('span', { hasText: 'AI Actions Used' })).toBeVisible();
  });

  test('should display Storage usage', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.locator('span', { hasText: 'Storage Used' })).toBeVisible();
  });

  test('should display management action buttons', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.locator('h3', { hasText: 'View Cost Details' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Change Plan' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Download Invoice' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Cancel Subscription' })).toBeVisible();
  });

  test('should return correct JSON payload from backend API', async ({ page, request }) => {
    await page.goto('/plan');


    const cookies = await page.context().cookies();
    //const authCookie = cookies.find(c => c.name === 'auth_token');
    await page.goto('/api/billing/my-plan');
    //expect(response.ok()).toBeTruthy();
    const data = await page.evaluate(() => JSON.parse(document.querySelector("body").innerText));


    expect(data).toHaveProperty('current_plan');
    expect(data).toHaveProperty('ai_actions_used');
    expect(data).toHaveProperty('storage_used_bytes');
    expect(data).toHaveProperty('next_bill_estimated');
  });

});
