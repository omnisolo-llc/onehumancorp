import { test, expect } from '@playwright/test';

test.describe('Omnichannel Cart & Tap-to-Pay Integration', () => {
  const tenantId = 'omni_test_tenant';

  test.beforeEach(async ({ page }) => {
    // Wait for the server to be ready and clear state
    await page.goto('/');
    await page.evaluate(() => {
      localStorage.clear();
      localStorage.setItem('tenant', 'omni_test_tenant');
    });

    await page.setViewportSize({ width: 375, height: 812 });

    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('omni_user@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();

    // Wait for dashboard to load after login to ensure session is valid
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
  });

  test('Owner creates an in-store cart, adds an item, and initiates Tap-to-Pay', async ({ page }) => {
    // We already navigated and logged in from beforeEach, now go to products/new
    await page.goto('/products/new');
    await page.getByPlaceholder('e.g., Guitar lessons for beginners, 1 hour').fill('Summer Dress');
    await page.getByRole('button', { name: 'Generate' }).click();

    // The AI takes a moment to generate
    await expect(page.getByRole('button', { name: 'Looks Good' })).toBeVisible({ timeout: 15000 });

    await page.getByRole('button', { name: 'Looks Good' }).click();
    await expect(page.getByText('Product Published!')).toBeVisible({ timeout: 10000 });

    // Grab the actual product ID from the product list page
    await page.goto('/products');
    const productLink = page.locator('a', { hasText: 'Summer Dress' }).first();
    await expect(productLink).toBeVisible({ timeout: 10000 });
    const href = await productLink.getAttribute('href');
    expect(href).toBeTruthy();
    const productId = href!.split('/').pop()!;
    expect(productId).toBeTruthy();

    // Now proceed to the POS Omnichannel page
    await page.goto('/pos/omnichannel');

    await expect(page.locator('text=New In-Store Sale')).toBeVisible();

    // 1. Create the Omnichannel Cart
    const createBtn = page.locator('#create-cart-btn');
    await expect(createBtn).toBeVisible();
    await createBtn.click();

    // Verify Cart is created successfully
    await expect(page.locator('#status-message')).toContainText('Cart created', { timeout: 10000 });
    await expect(page.locator('text=Cart ID:')).toBeVisible();

    // 2. Add an item (Summer Dress)
    const productInput = page.locator('#product-input');
    await expect(productInput).toBeVisible();
    await productInput.fill(productId);

    const addItemBtn = page.locator('#add-item-btn');
    await expect(addItemBtn).toBeVisible();
    await addItemBtn.click();

    // Wait for the item to be added
    await expect(page.locator('#status-message')).toContainText('Item added successfully', { timeout: 10000 });
    await expect(page.locator('text=Total Due')).toBeVisible();

    // Check if the price UI correctly reflects any item addition without hardcoding
    await expect(page.locator('text=Collect Payment')).toBeVisible({ timeout: 10000 });

    // 3. Verify Stripe Terminal Client renders
    const tapToPayContainer = page.locator('#tap-to-pay-container');
    await expect(tapToPayContainer).toBeVisible();

    // Verify Terminal initialized
    await expect(page.locator('text=Tap to Pay via Terminal')).toBeVisible();

    // It takes a little bit for the terminal to initialize
    await expect(tapToPayContainer.locator('text=Terminal initialized. Ready to discover readers.')).toBeVisible({ timeout: 10000 });

    // Click "Discover Readers"
    const discoverBtn = tapToPayContainer.locator('button:has-text("Discover Readers")');
    await expect(discoverBtn).toBeVisible();
    await discoverBtn.click();

    // In our mock environment, simulated=true discovers a mock reader
    await expect(tapToPayContainer.locator('text=Discovered')).toBeVisible({ timeout: 10000 });

    // Connect to the mock reader
    const connectBtn = tapToPayContainer.locator('button:has-text("Connect")').first();
    await expect(connectBtn).toBeVisible();
    await connectBtn.click();

    // Verify connection success and session started
    await expect(tapToPayContainer.locator('text=Connected to reader')).toBeVisible({ timeout: 10000 });

    // Verify Charge button appears
    const chargeBtn = tapToPayContainer.locator('button:has-text("Collect Payment")');
    await expect(chargeBtn).toBeVisible();

    // Verify the payment process flow (creates intent -> processes -> commits inventory)
    await chargeBtn.click();

    // Let the mock payment flow complete
    await expect(tapToPayContainer.locator('text=Payment successful!')).toBeVisible({ timeout: 15000 });
  });
});
