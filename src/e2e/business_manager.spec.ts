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

    // Should display the 3 dummy products from main.rs
    await expect(page.locator('text=Custom Vegan Cake')).toBeVisible();
    await expect(page.locator('text=Website Template')).toBeVisible();
    await expect(page.locator('text=Plumbing Repair')).toBeVisible();

    // Verify badges
    await expect(page.locator('text=5 in stock')).toBeVisible();
    await expect(page.locator('text=⚠️ Out of Stock')).toBeVisible();

    // Verify prices
    await expect(page.locator('text=$40.00')).toBeVisible();
    await expect(page.locator('text=$150.00')).toBeVisible();

    // Edit/Archive buttons should exist and have proper touch target size
    const editBtn = page.locator('button:has-text("Edit")').first();
    const archiveBtn = page.locator('button:has-text("Archive")').first();

    await expect(editBtn).toBeVisible();
    await expect(archiveBtn).toBeVisible();

    const editBox = await editBtn.boundingBox();
    expect(editBox?.height).toBeGreaterThanOrEqual(44);

    const archiveBox = await archiveBtn.boundingBox();
    expect(archiveBox?.height).toBeGreaterThanOrEqual(44);
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
    await expect(page.locator('text=What type of offering are you creating?')).toBeVisible();
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
  });


  test('should flow correctly for Digital Item creation', async ({ page }) => {
    await page.locator('text="+ Add New Offering"').click();
    await page.locator('text=💻 Digital Download').click();
    await page.locator('text="Next →"').click();
    await expect(page.locator('text=Details')).toBeVisible();
    await page.fill('input[placeholder="E.g. Custom Vegan Cake"]', 'Test Digital Product');
    await page.fill('input[placeholder="Brief description"]', 'A downloadable PDF');
    await page.fill('input[placeholder="0.00"]', '9.99');
    await expect(page.locator('text=Duration (minutes)')).toBeHidden();
    await page.locator('text="Create"').click();
    await expect(page.locator('text=My Offerings')).toBeVisible();
  });

  test('should flow correctly for Service Item creation', async ({ page }) => {
    await page.locator('text="+ Add New Offering"').click();
    await page.locator('text=⏱️ My Time / Service').click();
    await page.locator('text="Next →"').click();
    await expect(page.locator('text=Details')).toBeVisible();
    await page.fill('input[placeholder="E.g. Custom Vegan Cake"]', 'Test Service Booking');
    await page.fill('input[placeholder="Brief description"]', '1 hour consultation');
    await page.fill('input[placeholder="0.00"]', '100.00');
    await expect(page.locator('text=Duration (minutes)')).toBeVisible();
    await page.fill('input[placeholder="60"]', '60');
    await page.fill('input[placeholder="e.g. Mon-Fri 9am-5pm"]', 'Mon-Fri 9am-5pm');
    await page.locator('text="Create"').click();
    await expect(page.locator('text=My Offerings')).toBeVisible();
  });
});
