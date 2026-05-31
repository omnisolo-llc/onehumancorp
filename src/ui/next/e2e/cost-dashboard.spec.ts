import { test, expect } from '@playwright/test';

test.describe('Cost Dashboard and My Plan Flow', () => {
  test('navigates from dashboard to my plan, then to cost dashboard, verifying UI elements', async ({ page }) => {
    // Navigate to plan page
    await page.goto('http://localhost:3000/plan');

    // Wait for the My Plan page to load
    await expect(page.locator('h1:has-text("My Plan")')).toBeVisible({ timeout: 5000 });

    // Verify some UI elements in My Plan page
    await expect(page.locator('h2:has-text("Current Plan")')).toBeVisible();
    await expect(page.locator('h2:has-text("Your Current Usage")')).toBeVisible();
    await expect(page.locator('span:has-text("AI Actions Used")')).toBeVisible();
    await expect(page.locator('span:has-text("Storage Used")')).toBeVisible();

    // Find the button to View Cost Details and click it
    const costDetailsButton = page.locator('h3:has-text("View Cost Details")');
    await expect(costDetailsButton).toBeVisible();
    await costDetailsButton.click();

    // Verify it navigates to Cost Dashboard page
    await page.waitForURL('**/cost-dashboard');
    await expect(page.locator('h1:has-text("Business Advisory Dashboard")')).toBeVisible({ timeout: 5000 });

    // Verify Cost Dashboard UI elements
    await expect(page.locator('h2:has-text("Cost Transparency")')).toBeVisible();
    await expect(page.locator('h2:has-text("Total Costs")')).toBeVisible();
    await expect(page.locator('h2:has-text("Cost Breakdown")')).toBeVisible();
    await expect(page.locator('span:has-text("LLM Usage")')).toBeVisible();
    await expect(page.locator('span:has-text("Storage")')).toBeVisible();
    await expect(page.locator('span:has-text("Payment Fees")')).toBeVisible();

    // Click back to My Plan
    const backToPlanBtn = page.locator('button:has-text("Back to My Plan")');
    await expect(backToPlanBtn).toBeVisible();
    await backToPlanBtn.click();
    await page.waitForURL('**/plan');

    // Verify we are back on My Plan
    await expect(page.locator('h1:has-text("My Plan")')).toBeVisible();
  });
});
