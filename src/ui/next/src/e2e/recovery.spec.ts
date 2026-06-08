import { expect, test } from '@playwright/test';

test.describe('Autonomous AI Cart Recovery', () => {
  test('displays recovery dashboard and approves draft', async ({ page }) => {
    // Navigate directly to the cart recovery page since it's the focus of this task
    await page.goto('/cart-recovery');

    // Verify the page loads and displays the correct title
    await expect(page.locator('h1')).toContainText('Abandoned Cart Recovery');

    // Fill in the optional context inputs
    await page.fill('input#customer-name', 'Sarah');
    await page.fill('input#cart-value', '$120.00');

    // Generate AI draft
    await page.click('button:has-text("Generate AI Campaign")');

    // Verify draft was generated and is visible
    await expect(page.locator('pre')).toContainText('Sarah');
    await expect(page.locator('pre')).toContainText('$120.00');

    // Approve and send the draft
    await page.click('button:has-text("Send to")');

    // Verify success message
    await expect(page.locator('text=Campaign sent to')).toBeVisible();
  });
});
