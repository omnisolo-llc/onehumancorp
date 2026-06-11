import { test, expect } from '@playwright/test';

test.describe('Omnichannel Cart & Tap-to-Pay Integration', () => {
  const tenantId = 'omni_test_tenant';

  test.beforeEach(async ({ request }) => {
    // Setup initial product for the cart
    await request.post('/api/v1/builder/seeder/exec', {
      data: {
        sql: `INSERT INTO products (id, tenant_id, title, type, price_cents, inventory_count)
              VALUES ('prod_omni_dress', '${tenantId}', 'Summer Dress', 'physical', 4500, 10)
              ON CONFLICT (id) DO UPDATE SET inventory_count = 10`
      },
    });
  });

  test('Owner creates an in-store cart, adds an item, and initiates Tap-to-Pay', async ({ page, browser }) => {
    // Navigate to the POS Omnichannel page
    await page.evaluate((t) => localStorage.setItem('tenant', t), tenantId);
    await page.goto('/pos/omnichannel');

    await expect(page.locator('text=New In-Store Sale')).toBeVisible();

    // 1. Create the Omnichannel Cart
    const createBtn = page.locator('#create-cart-btn');
    await expect(createBtn).toBeVisible();
    await createBtn.click();

    // Verify Cart is created successfully
    await expect(page.locator('#status-message')).toContainText('Cart created');
    await expect(page.locator('text=Cart ID:')).toBeVisible();

    // 2. Add an item (Summer Dress)
    const productInput = page.locator('#product-input');
    await productInput.fill('prod_omni_dress');

    const addItemBtn = page.locator('#add-item-btn');
    await addItemBtn.click();

    // Wait for the item to be added
    await expect(page.locator('#status-message')).toContainText('Item added successfully');
    await expect(page.locator('text=Total Due')).toBeVisible();
    await expect(page.locator('text=$15.00')).toBeVisible(); // 1500 cents from mock payload for now

    // 3. Verify Stripe Terminal Client renders
    const tapToPayContainer = page.locator('#tap-to-pay-container');
    await expect(tapToPayContainer).toBeVisible();

    // Verify Terminal initialized
    await expect(page.locator('text=Tap to Pay via Terminal')).toBeVisible();

    // It takes a little bit for the terminal to initialize
    await expect(tapToPayContainer.locator('text=Terminal initialized. Ready to discover readers.')).toBeVisible({ timeout: 10000 });

    // Click "Discover Readers"
    const discoverBtn = tapToPayContainer.locator('button:has-text("Discover Readers")');
    await discoverBtn.click();

    // In our mock environment, simulated=true discovers a mock reader
    await expect(tapToPayContainer.locator('text=Discovered')).toBeVisible({ timeout: 10000 });

    // Connect to the mock reader
    const connectBtn = tapToPayContainer.locator('button:has-text("Connect")').first();
    await connectBtn.click();

    // Verify connection success and session started
    await expect(tapToPayContainer.locator('text=Connected to reader')).toBeVisible({ timeout: 10000 });

    // Verify Charge button appears
    const chargeBtn = tapToPayContainer.locator('button:has-text("Charge $15.00")');
    await expect(chargeBtn).toBeVisible();

    // Verify the payment process flow (creates intent -> processes -> commits inventory)
    await chargeBtn.click();

    // Let the mock payment flow complete
    await expect(tapToPayContainer.locator('text=Payment successful!')).toBeVisible({ timeout: 15000 });
  });
});
