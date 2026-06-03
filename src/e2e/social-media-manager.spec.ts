import { test, expect } from './fixtures';

test.describe('Autonomous Social Media Manager Agent E2E', () => {
  test('Creating a product automatically drafts a social media post for approval', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    // Click on Add Product via mobile bottom nav
    const addProductBtn = page.locator('#mobile-bottom-nav .nav-item').filter({ hasText: 'Add Product' });
    await addProductBtn.click();

    // Fill out the product form
    await page.fill('#item-name', 'Vegan Chocolate Cake');
    await page.fill('#item-price', '25.00');
    await page.fill('#item-desc', 'Delicious, rich, and 100% vegan chocolate cake.');

    // Accept the alert dialog "Saved Vegan Chocolate Cake successfully!"
    page.once('dialog', dialog => dialog.accept());

    // Save the item
    await page.click('button:has-text("Save Item")');

    // Wait for the Dashboard to load and network to settle
    await page.waitForSelector('#dashboard-screen');

    // Check for the new draft in the pending approvals hub
    await page.goto('/?screen=dashboard-screen');
    await page.waitForLoadState('networkidle');

    // We expect the Marketing agent to have drafted an Instagram post
    // Let's locate the Approval Card
    // We might need to wait for it as it goes through the orchestration queue
    await expect(page.locator('text=Marketing')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Draft Instagram post for Vegan Chocolate Cake')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=social_post')).toBeVisible({ timeout: 10000 });

    // Click Approve
    const approveButton = page.locator('button:has-text("Approve")').first();
    await approveButton.click();

    // The approval should disappear from the pending list
    await expect(page.locator('text=Draft Instagram post for Vegan Chocolate Cake')).not.toBeVisible({ timeout: 10000 });
  });
});
