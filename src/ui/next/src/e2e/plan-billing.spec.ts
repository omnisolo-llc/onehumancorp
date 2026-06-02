import { test, expect } from '@playwright/test';

test.describe('Plan and Billing CUJ', () => {
  test('My Plan page loads and shows pricing option', async ({ page }) => {
    // 1. Owner starts at dashboard, goes to plan
    await page.goto('http://localhost:3000/plan');

    // 2. Verify page loads
    await expect(page.locator('h1', { hasText: 'My Plan' })).toBeVisible({ timeout: 15000 });

    // 3. Verify plan data renders correctly
    await expect(page.locator('#my-plan-name')).toBeVisible();
    await expect(page.locator('#my-plan-next-bill')).toBeVisible();

    // 4. Click 'View Upgrade Plans' and ensure pricing page loads
    await page.locator('button', { hasText: 'View Upgrade Plans' }).click();
    await expect(page).toHaveURL('http://localhost:3000/pricing');

    // 5. Verify pricing tiers are visible
    await expect(page.locator('h3', { hasText: 'Free' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Starter' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Pro' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Business' })).toBeVisible();
  });

  test('My Plan displays AI Actions Used progress', async ({ page }) => {
    await page.goto('http://localhost:3000/plan');
    await expect(page.locator('h1', { hasText: 'My Plan' })).toBeVisible({ timeout: 15000 });

    // Check AI actions used header
    await expect(page.locator('span', { hasText: 'AI Actions Used' })).toBeVisible();

    // There should be a visual bar and a fraction limit displayed (e.g. 0 / 100)
    await expect(page.locator('text=/[0-9]+\\s*\\/\\s*([0-9]+|Unlimited)/i').first()).toBeVisible();
  });

  test('My Plan displays Storage Used progress', async ({ page }) => {
    await page.goto('http://localhost:3000/plan');
    await expect(page.locator('h1', { hasText: 'My Plan' })).toBeVisible({ timeout: 15000 });

    // Check Storage used header
    await expect(page.locator('span', { hasText: 'Storage Used' })).toBeVisible();

    // Check for formatted MB/GB or unlimited (relaxing regex to match "< 1 MB / 500 MB")
    await expect(page.locator('text=/.*MB.*\\/.*(MB|GB|Unlimited).*/i').first()).toBeVisible();
  });

  test('My Plan navigation to Cost Dashboard', async ({ page }) => {
    await page.goto('http://localhost:3000/plan');
    await expect(page.locator('h1', { hasText: 'My Plan' })).toBeVisible({ timeout: 15000 });

    // Click View Cost Details button
    const viewCostBtn = page.locator('button', { hasText: 'View Cost Details' });
    await expect(viewCostBtn).toBeVisible();
    await viewCostBtn.click();

    // Verify navigation
    await expect(page).toHaveURL('http://localhost:3000/cost-dashboard');
    await expect(page.locator('h1', { hasText: 'Business Advisory Dashboard' })).toBeVisible({ timeout: 15000 });
  });

  test('My Plan change plan button navigates to pricing', async ({ page }) => {
    await page.goto('http://localhost:3000/plan');
    await expect(page.locator('h1', { hasText: 'My Plan' })).toBeVisible({ timeout: 15000 });

    // Click Change Plan button
    const changePlanBtn = page.locator('button', { hasText: 'Change Plan' });
    await expect(changePlanBtn).toBeVisible();
    await changePlanBtn.click();

    // Verify navigation
    await expect(page).toHaveURL('http://localhost:3000/pricing');
    await expect(page.locator('h1', { hasText: 'Pricing Plans' })).toBeVisible({ timeout: 15000 });
  });
});
