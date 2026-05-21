import { test, expect } from './fixtures';

test.describe('OHC Growth Strategy Features', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the dashboard from home page
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test('Unified Product & Service Catalog allows adding items', async ({ page }) => {
    // 1. Verify Unified Catalog is present
    const unifiedCatalog = page.locator('#unified-catalog');
    await expect(unifiedCatalog).toBeVisible();

    // 2. Click Add New Item
    await page.locator('text="Add New Item"').click();
    await expect(page.locator('#catalog-form')).toBeVisible();

    // 3. Select type and fill form
    await page.locator('#catalog-type').selectOption('Service');

    // Simulate AI generation click
    await page.locator('text="Generate AI Description from Image"').click();
    await expect(page.locator('#catalog-name')).toHaveValue('Generated Item');
    await expect(page.locator('#catalog-price')).toHaveValue('$99');

    // 4. Save item
    await page.locator('text="Save Item"').click();

    // 5. Verify the new item appears in the list
    const catalogList = page.locator('#catalog-list');
    await expect(catalogList).toContainText('Generated Item');
    await expect(catalogList).toContainText('Service');
    await expect(catalogList).toContainText('$99');
  });

  test('WhatsApp & IG DM Order Ingestion approval', async ({ page }) => {
    // 1. Verify the specific approval item is present
    const approvalItem = page.locator('#approval-item-2');
    await expect(approvalItem).toBeVisible();
    await expect(approvalItem).toContainText('New order request from Instagram');

    // Setup an alert dialog listener
    let dialogMessage = '';
    page.on('dialog', dialog => {
      dialogMessage = dialog.message();
      dialog.accept();
    });

    // 2. Click Approve
    await approvalItem.locator('text="Approve"').click();

    // 3. Verify it's approved and UI is updated
    expect(dialogMessage).toBe('Order approved');
    await expect(approvalItem).toBeHidden();
  });

  test('One-Click AI Insights action', async ({ page }) => {
    // 1. Verify the weekly insights card is present
    const insightsCard = page.locator('#weekly-insights');
    await expect(insightsCard).toBeVisible();
    await expect(insightsCard).toContainText('Sales are slow this week');

    // Setup an alert dialog listener
    let dialogMessage = '';
    page.on('dialog', dialog => {
      dialogMessage = dialog.message();
      dialog.accept();
    });

    // 2. Click Yes, Do It
    await insightsCard.locator('text="Yes, Do It"').click();

    // 3. Verify action is taken and UI is updated
    expect(dialogMessage).toBe('Discount code emailed!');
    await expect(insightsCard).toBeHidden();
  });
});
