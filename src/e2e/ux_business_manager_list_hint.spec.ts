import { test, expect } from '@playwright/test';

test.describe('Business Manager UI - First-Time User Tour', () => {
  test.beforeEach(async ({ page }) => {
    // 1. Start from home page after login
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();

    // 2. Navigate to Dashboard (assuming login goes to dashboard)
    await page.waitForURL('**/*');

    // Find and click the Add Product button
    const addProductBtn = page.locator('button:has-text("Add")');
    await expect(addProductBtn).toBeVisible();
    await addProductBtn.click();
  });

  test('should display the contextual list hint when ? is clicked', async ({ page }) => {
    // Should show "My Offerings" title
    await expect(page.locator('text=My Offerings')).toBeVisible();

    // The hint should not be visible initially
    await expect(page.locator('text=Manage the items and services you sell here.')).toBeHidden();

    // Click the '?' hint button
    const hintBtn = page.locator('button:has-text("?")').first();
    await expect(hintBtn).toBeVisible();
    await hintBtn.click();

    // The hint should now be visible
    await expect(page.locator('text=Manage the items and services you sell here.')).toBeVisible();

    // Click it again to toggle off
    await hintBtn.click();

    // The hint should be hidden again
    await expect(page.locator('text=Manage the items and services you sell here.')).toBeHidden();
  });

  test('should retain the hint toggle state when navigating back and forth', async ({ page }) => {
    // Click the '?' hint button to turn it on
    const hintBtn = page.locator('button:has-text("?")').first();
    await hintBtn.click();
    await expect(page.locator('text=Manage the items and services you sell here.')).toBeVisible();

    // Navigate to Add view
    const addNewBtn = page.locator('text="+ Add New Offering"');
    await addNewBtn.click();
    await expect(page.locator('text=Add Offering')).toBeVisible();

    // Navigate back to list view
    const backBtn = page.locator('button:has-text("Back to List")');
    await backBtn.click();

    // The hint should still be visible because we toggled it on
    await expect(page.locator('text=Manage the items and services you sell here.')).toBeVisible();
  });

  test('Back to list and Close button tap targets are at least 44px', async ({ page }) => {
    // Navigate to Add view to see the Back button
    await page.locator('text="+ Add New Offering"').click();

    const backBtn = page.locator('button:has-text("Back to List")');
    await expect(backBtn).toBeVisible();

    const backBox = await backBtn.boundingBox();
    expect(backBox?.height).toBeGreaterThanOrEqual(44);

    // Navigate back to list view to see the Close button
    await backBtn.click();

    const closeBtn = page.locator('button:has-text("Close")');
    await expect(closeBtn).toBeVisible();

    const closeBox = await closeBtn.boundingBox();
    expect(closeBox?.height).toBeGreaterThanOrEqual(44);
  });

  test('First-time tour hint text contains no jargon', async ({ page }) => {
    // Click the '?' hint button
    await page.locator('button:has-text("?")').first().click();

    // The text should be plain language
    const hintText = await page.locator('text=Manage the items and services you sell here.').textContent();
    expect(hintText).toContain('items and services');
    expect(hintText).not.toContain('products');
    expect(hintText).not.toContain('entities');
    expect(hintText).not.toContain('database');
  });

  test('hint toggle should have a tap target of at least 44x44', async ({ page }) => {
    const hintBtn = page.locator('button:has-text("?")').first();
    await expect(hintBtn).toBeVisible();

    const box = await hintBtn.boundingBox();
    expect(box?.height).toBeGreaterThanOrEqual(44);
    expect(box?.width).toBeGreaterThanOrEqual(44);
  });
});
