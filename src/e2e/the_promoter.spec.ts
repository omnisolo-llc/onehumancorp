import { test, expect } from './fixtures';

test.describe('The Promoter Agent', () => {
  test('should create a draft social post when a new product is added', async ({ page }) => {
    // Need to login first
    await page.goto('/login');
    await page.fill('input[placeholder="Email or Username"]', 'Maya');
    await page.getByRole('button', { name: 'Log In' }).click();

    // 1. Go to Inventory / Products page
    await page.goto('/inventory');

    // 2. Add a new product
    await page.getByRole('button', { name: 'Add Product' }).click();
    await page.getByLabel('Name').fill('Vegan Chocolate Cake');
    await page.getByLabel('Description').fill('A delicious, moist vegan chocolate cake.');
    await page.getByLabel('Price').fill('45.00');
    await page.getByRole('button', { name: 'Save Product' }).click();

    // 3. Go to Team/Assistant page (where ApprovalInbox is)
    await page.goto('/team');

    // 4. Verify that The Promoter drafted a post
    // The Promoter's task should appear in the ApprovalInbox
    await expect(page.getByText('Social Post Drafted')).toBeVisible({ timeout: 15000 });
    await expect(page.getByText('Vegan Chocolate Cake')).toBeVisible();

  });
});
