import { test, expect } from '@playwright/test';

test.describe('Dashboard UX Friction Fix Verification', () => {
  test('Grandmother Test: User navigates smoothly without jargon', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Test 1: Verify the label is Business Health, not Store Rating or Store Health
    const businessHealth = page.locator('text="Business Health"');
    await expect(businessHealth.first()).toBeVisible();

    // Test 2: The tooltip for Business Health should be clear and descriptive
    const helpBtn = page.locator('button:has-text("? Learn about Business Health")').first();
    const tooltipText = page.locator('text="Your Business Health is an AI-calculated score of your business\'s overall health and performance."');
    if (await helpBtn.isVisible()) {
      await helpBtn.click();
      await expect(tooltipText).toBeVisible();
    }
  });

  test('Plain language labels consistency check', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');

    // Test 3: Today's Sales is clear
    const todaysSales = page.locator('text="Today\'s Sales"');
    await expect(todaysSales.first()).toBeVisible();

    // Test 4: My Store label is present
    const myStore = page.locator('text="My Store"');
    await expect(myStore.first()).toBeVisible();

    // Test 5: Verify no "Revenue TTD" jargon
    const oldRevenue = page.locator('text="Revenue TTD"');
    await expect(oldRevenue).toHaveCount(0);
  });

  test('Add Product action navigates directly to Add Offering view', async ({ page }) => {
    // 1. Start from home page after login
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();

    await page.waitForURL('**/*');

    // Find and click the Add Product button
    const addProductBtn = page.locator('button:has-text("Add Product")');
    await expect(addProductBtn).toBeVisible();
    await addProductBtn.click();

    // It should open the Business Manager directly to the "Add Offering" view (Step 0)
    await expect(page.locator('text=Add Offering')).toBeVisible();
    await expect(page.locator('text=What type of offering are you creating?')).toBeVisible();
  });

  test('Add Product quick action is accessible and visible immediately on dashboard load', async ({ page }) => {
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();
    await page.waitForURL('**/*');
    const addProductBtn = page.locator('button:has-text("Add Product")');
    await expect(addProductBtn).toBeVisible();
  });

  test('Add Product action uses premium design standards and correct size', async ({ page }) => {
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();
    await page.waitForURL('**/*');
    const addProductBtn = page.locator('button:has-text("Add Product")');
    const box = await addProductBtn.boundingBox();
    expect(box?.height).toBeGreaterThanOrEqual(44);
  });

  test('Add Product opens "Add Offering" with physical product type clearly defined', async ({ page }) => {
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();
    await page.waitForURL('**/*');
    await page.locator('button:has-text("Add Product")').click();
    await expect(page.locator('text=📦 Physical Item')).toBeVisible();
    await expect(page.locator('text=Shipped to customers')).toBeVisible();
  });

  test('Add Product provides clear help tooltip without jargon', async ({ page }) => {
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();
    await page.waitForURL('**/*');
    await page.locator('button:has-text("Add Product")').click();

    // We already know it opens BusinessManager correctly. Just verify the help button is there.
    const helpBtn = page.locator('button:has-text("?")').first();
    await helpBtn.click();
    await expect(page.locator('text="Choose Physical for items you ship, Digital for files like PDFs, or Service for bookings and your time."')).toBeVisible();
  });
});
