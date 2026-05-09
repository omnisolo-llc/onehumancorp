import { test, expect } from '@playwright/test';

test.describe('Business Manager UI', () => {
  test.beforeEach(async ({ page }) => {
    // 1. Start from home page after login
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();

    // 2. Navigate to Dashboard (assuming login goes to dashboard)
    await page.waitForURL('**/*');

    // Find and click the Add Product button
    // The tooltip says "add_product" and the button text is "Add"
    const addProductBtn = page.locator('button:has-text("Add")');
    await expect(addProductBtn).toBeVisible();
    await addProductBtn.click();
  });

  test('should display "My Offerings" product list view', async ({ page }) => {
    // Should show "My Offerings" title instead of "Add Offering" initially
    await expect(page.locator('text=My Offerings')).toBeVisible();
  });

  test('should navigate to "Add Offering" view when "+ Add New Offering" is clicked', async ({ page }) => {
    // Verify initial list view
    await expect(page.locator('text=My Offerings')).toBeVisible();

    // Click Add New
    const addNewBtn = page.locator('text="+ Add New Offering"');
    await expect(addNewBtn).toBeVisible();
    await addNewBtn.click();

    // Verify we are now on the "Add Offering" view
    await expect(page.locator('text=Add Offering')).toBeVisible();
    await expect(page.locator('text=What are you offering?')).toBeVisible();
  });

  test('should navigate back to list view when "Back to List" is clicked', async ({ page }) => {
    // Navigate to Add
    await page.locator('text="+ Add New Offering"').click();
    await expect(page.locator('text=Add Offering')).toBeVisible();

    // Click Back to List
    const backBtn = page.locator('button:has-text("Back to List")');
    await expect(backBtn).toBeVisible();
    await backBtn.click();

    // Should be on List View
    await expect(page.locator('text=My Offerings')).toBeVisible();
  });

  test('should close the dialog when Close is clicked', async ({ page }) => {
    // Since current_view is list initially, Close should hide the window
    const closeBtn = page.locator('button:has-text("Close")');
    await expect(closeBtn).toBeVisible();
    await closeBtn.click();

    await expect(page.locator('text=My Offerings')).toBeHidden();
  });

  test('should flow correctly for Physical Item creation', async ({ page }) => {
    // Navigate to Add Offering
    await page.locator('text="+ Add New Offering"').click();

    // Click on Physical Item card
    await page.locator('text=📦 Physical Item').click();

    // Proceed to next step
    const nextBtn = page.locator('text="Next →"');
    await expect(nextBtn).toBeVisible();
    await nextBtn.click();

    // Verify step 1 fields
    await expect(page.locator('text=Details')).toBeVisible();

    // Fill out the form fields using the exact placeholder texts
    await page.fill('input[placeholder="E.g. Custom Vegan Cake"]', 'Test Physical Product');
    await page.fill('input[placeholder="Brief description"]', 'A high quality test item');
    await page.fill('input[placeholder="0.00"]', '19.99');

    // For physical items, Service fields should not be visible
    await expect(page.locator('text=Duration (minutes)')).toBeHidden();

    // Click Create
    const createBtn = page.locator('text="Create"');
    await expect(createBtn).toBeVisible();
    await createBtn.click();

    // Should go back to the list view after creation
    await expect(page.locator('text=My Offerings')).toBeVisible();

    // Verify the newly created product appears in the list (Full Stack Verification)
    await expect(page.locator('text=Test Physical Product')).toBeVisible();
    await expect(page.locator('text=$19.99')).toBeVisible();

    // Ensure that Edit/Archive buttons are shown for the new product
    const editBtn = page.locator('button:has-text("Edit")').first();
    const archiveBtn = page.locator('button:has-text("Archive")').first();

    await expect(editBtn).toBeVisible();
    await expect(archiveBtn).toBeVisible();

    const editBox = await editBtn.boundingBox();
    expect(editBox?.height).toBeGreaterThanOrEqual(44);

    const archiveBox = await archiveBtn.boundingBox();
    expect(archiveBox?.height).toBeGreaterThanOrEqual(44);
  });
