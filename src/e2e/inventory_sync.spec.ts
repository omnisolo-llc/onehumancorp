import { test, expect, loginAs } from './fixtures';

test.describe('Distributed Inventory Sync POS', () => {

  test('should lock inventory during POS transaction and prevent online checkout', async ({ browser }) => {
    const posContext = await browser.newContext();
    const onlineContext = await browser.newContext();

    const posPage = await posContext.newPage();
    const onlinePage = await onlineContext.newPage();

    // Log in normally
    await loginAs(posContext, posPage);

    // 1. Navigate to POS terminal
    await posPage.goto('/pos/terminal');
    await expect(posPage.locator('text=Terminal Locked')).toBeVisible({ timeout: 15000 });

    // Enter PIN: 1234
    await posPage.getByRole('button', { name: '1', exact: true }).click();
    await posPage.getByRole('button', { name: '2', exact: true }).click();
    await posPage.getByRole('button', { name: '3', exact: true }).click();
    await posPage.getByRole('button', { name: '4', exact: true }).click();

    await expect(posPage.locator('text=Test Owner')).toBeVisible();

    // Select product seeded in the DB
    await posPage.locator('text=Vegan Celebration Cake').click();

    // Verify "Charge $39.99" button is visible
    await expect(posPage.getByRole('button', { name: 'Charge $39.99' })).toBeVisible();

    // POS terminal applies lock when clicking charge. We'll wait a bit.
    await posPage.getByRole('button', { name: 'Charge $39.99' }).click();

    await expect(posPage.locator('text=Status: Reserving inventory...')).toBeVisible();
    await expect(posPage.locator('text=Status: Creating payment intent...')).toBeVisible();

    // 2. Online customer attempts checkout for the same item at the same time
    await onlinePage.goto('/checkout?product_id=e2e-product-cake');

    await expect(onlinePage.getByRole('button', { name: 'Pay with Stripe' })).toBeVisible();

    // Try to pay online
    await onlinePage.getByRole('button', { name: 'Pay with Stripe' }).click();

    // Should fail gracefully
    await expect(onlinePage.locator('text=Sorry, this item was just purchased in-store.')).toBeVisible();

    await posContext.close();
    await onlineContext.close();
  });
});

test.describe('Low Stock Restock Action Card', () => {
  test('should trigger low stock approval card when inventory drops to 5 or below after a valid POS sale', async ({ browser }) => {

    const posContext = await browser.newContext();
    const page = await posContext.newPage();

    await loginAs(posContext, page);

    // Create a product with 6 stock via UI flow instead of fetch
    await page.goto('/products/new');
    await page.getByLabel('Product Name').fill('Limited Edition Mug');
    await page.getByLabel('Price').fill('15');
    await page.getByLabel('Inventory Count').fill('6');
    await page.getByRole('button', { name: 'Save Product' }).click();

    // wait for save
    await expect(page.locator('text=Product created successfully')).toBeVisible();

    await page.goto('/pos/terminal');
    await expect(page.locator('text=Terminal Locked')).toBeVisible({ timeout: 15000 });

    // Enter PIN: 1234
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    await expect(page.locator('text=Test Owner')).toBeVisible();

    // Select product
    await page.locator('text=Limited Edition Mug').click();
    await page.getByRole('button', { name: 'Charge $15.00' }).click();

    await expect(page.locator('text=Status: Creating payment intent...')).toBeVisible();
    await expect(page.locator('text=Status: Payment successful!')).toBeVisible();

    // 2. Navigate to the Team/Approval Inbox to verify the new card
    await page.goto('/team/chat');

    await page.getByRole('button', { name: 'Operations' }).click();

    // We expect the low stock alert to now be generated and visible because stock dropped to 5
    await expect(page.locator('text=Low Stock Alert')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=Remaining Stock:')).toBeVisible();
    await expect(page.locator('text=5').first()).toBeVisible(); // stock should be 5

    await posContext.close();
  });
});
