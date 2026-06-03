import { test, expect } from './fixtures';

test.describe('Zero-Config AI-Powered Subscriptions', () => {
  test('merchant can create a subscription product and customer can subscribe', async ({ page, request }) => {
    // 1. Merchant creates a subscription product
    await page.goto('/products/new');
    await expect(page.getByRole('heading', { name: 'Add Product' })).toBeVisible();

    // Simulate image upload (triggers AI loading state)
    const fileChooserPromise = page.waitForEvent('filechooser');
    // Click the label directly since the input is hidden
    await page.getByText('Take a photo or upload').click();
    const fileChooser = await fileChooserPromise;
    // create a dummy file for the filechooser
    await fileChooser.setFiles({
      name: 'cake.jpg',
      mimeType: 'image/jpeg',
      buffer: Buffer.from('dummy image data')
    });

    // AutoDream AI loading state
    await expect(page.getByText('AutoDream AI is analyzing your photo...')).toBeVisible();

    // Wait for the mock API to complete the loading state and show the product form
    await expect(page.getByText('AI Generated')).toBeVisible({ timeout: 10000 });

    // Verify AI generated fields are populated
    await expect(page.locator('input[type="text"]').first()).toHaveValue(/./);

    // Scroll down to the subscription toggle
    const toggle = page.getByText('Offer as Subscription');
    await toggle.scrollIntoViewIfNeeded();
    await page.waitForTimeout(500); // Give it time to settle after scroll

    // Merchant toggles "Offer as Subscription"
    await toggle.click({ force: true }); // Use force: true in case it's partially hidden by a sticky header

    // Configure Subscription options
    await expect(page.getByText('Deliver every')).toBeVisible();
    await page.locator('select').selectOption('Month');

    // Publish product
    await page.getByRole('button', { name: 'Publish Product' }).click();
    await expect(page.getByText('Product Published!')).toBeVisible();

    // 2. The Customer Journey
    // Now we navigate to a specific checkout page for the mock
    await page.goto('/checkout?type=subscription&interval=Month&product=Vegan%20Celebration%20Cake&price=3999');
    await expect(page.getByRole('heading', { name: 'Checkout' })).toBeVisible();

    // Verify it shows subscription formatting
    await expect(page.getByText('$39.99 / Month')).toBeVisible();

    // The user taps Pay Now
    const payNowBtn = page.getByRole('button', { name: 'Pay Now' });
    await expect(payNowBtn).toBeVisible();

    await payNowBtn.click();

    // 3. Verify it processes successfully
    await expect(page.getByText('Payment Successful!')).toBeVisible();
    await expect(page.getByText(/magic link/)).toBeVisible();

    // 4. Test the API directly to ensure the data structure works as expected
    const apiResponse = await request.post('/api/v1/billing/subscriptions', {
      data: {
        tenant_id: 'e2e-tenant',
        customer_id: 'e2e-customer-ava',
        product_id: 'e2e-product-cake',
        interval: 'Month',
        price_cents: 3999
      }
    });

    expect(apiResponse.ok()).toBeTruthy();
    const data = await apiResponse.json();
    expect(data.success).toBe(true);
    expect(data.subscription_id).toContain('sub_');
    expect(data.magic_link).toContain('magic_');
  });
});
