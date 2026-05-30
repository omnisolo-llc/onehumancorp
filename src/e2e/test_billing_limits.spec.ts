import { test as baseTest, expect } from '@playwright/test';

// Override test to bypass the local loginAs fixture which assumes a backend
const test = baseTest.extend({
  page: async ({ page }, use) => {
    // Just pass the raw page
    await use(page);
  },
});

test.describe('Billing & Rate Limits', () => {
  test('should view the pricing page without errors', async ({ page }) => {
    await page.goto('http://localhost:3000/pricing');
    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Free', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Starter', exact: true })).toBeVisible();
  });

  test('should view the plan page without errors', async ({ page }) => {
    await page.goto('http://localhost:3000/plan');
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();
    await expect(page.getByText('Current Plan')).toBeVisible();
    await expect(page.getByText('Free', { exact: true })).toBeVisible();
  });

  test('should view cost dashboard without errors', async ({ page }) => {
    await page.goto('http://localhost:3000/cost-dashboard');
    await expect(page.getByRole('heading', { name: 'Business Advisory Dashboard' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Advisory Summary' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Cost Transparency' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Cost Breakdown' })).toBeVisible();
  });
});
