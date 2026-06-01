import { test, expect } from './fixtures';

test.describe('Viral Trial Extension E2E', () => {
  test('allows the user to extend their free trial by completing growth tasks', async ({ page }) => {
    // Navigate to the dashboard
    page.on('console', msg => console.log('BROWSER CONSOLE:', msg.text()));
    await page.goto('/dashboard');

    // Ensure the dashboard is visible
    await expect(page.getByRole('heading', { name: 'Extend Your Trial' })).toBeVisible();

    // Verify the initial trial days (14 days)
    const daysLeftLocator = page.locator('.text-5xl.font-outfit.font-bold.text-gray-900').first();
    await expect(daysLeftLocator).toContainText('14');

    // Click the "Connect Twitter" button
    const connectTwitterButton = page.locator('#connect-twitter-btn');
    await connectTwitterButton.click();

    // Verify the trial days increase to 21
    await expect(daysLeftLocator).toContainText('21');

    // Verify the button changes to "Connected"
    await expect(page.getByRole('button', { name: 'Connected' }).first()).toBeVisible();
    await expect(page.getByRole('button', { name: 'Connected' }).first()).toBeDisabled();

    // Click the "Leave a Review" button
    const leaveReviewButton = page.locator('#leave-review-btn');
    await leaveReviewButton.click();

    // Verify the trial days increase to 28
    await expect(daysLeftLocator).toContainText('28');

    // Verify the button changes to "Done"
    await expect(page.getByRole('button', { name: 'Done' }).first()).toBeVisible();

    // Click the "Add First Product" via the top "Add Item" button
    const addItemButton = page.locator('#add-item-btn');
    await addItemButton.click();

    // The "Add Item Modal" appears
    await expect(page.getByRole('heading', { name: 'Add New Item' })).toBeVisible();

    // Click "Save Product"
    const saveProductButton = page.locator('#add-item-modal button');
    await saveProductButton.click();

    // The modal should close and the "Add First Product" task should be marked as "Done"
    // Verify the trial days increase to 35
    await expect(daysLeftLocator).toContainText('35');

    // We should now have two "Done" buttons (Review, Add First Product)
    const doneButtons = page.getByRole('button', { name: 'Done' });
    await expect(doneButtons).toHaveCount(2);
  });
});
