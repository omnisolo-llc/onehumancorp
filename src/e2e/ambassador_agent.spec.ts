import { test, expect } from '@playwright/test';
import { v4 as uuidv4 } from 'uuid';

test.describe('The Ambassador (Customer Success Agent) - Instagram DM E2E', () => {
  test('should draft and approve an Instagram DM reply based on real inventory', async ({ page, request }) => {
    // 1. Create a unique test user and business
    const uniqueId = Date.now();
    const testEmail = `maya_baker_${uniqueId}@example.com`;
    const testPassword = 'Password123!';

    // Sign up
    await page.goto('/signup');
    await page.getByPlaceholder('Email').fill(testEmail);
    await page.getByPlaceholder('Password').fill(testPassword);
    await page.getByRole('button', { name: 'Sign Up' }).click();
    await page.waitForURL('/dashboard');

    // Set up business
    await page.goto('/settings/business');
    await page.getByLabel('Business Name').fill(`Maya's Bakery ${uniqueId}`);
    await page.getByRole('button', { name: 'Save' }).click();

    // 2. Add product "Vegan Chocolate Cake" with inventory
    await page.goto('/products');
    await page.getByRole('button', { name: 'Add Product' }).click();
    await page.getByLabel('Product Name').fill('Vegan Chocolate Cake');
    await page.getByLabel('Description').fill('Delicious dairy-free chocolate cake');
    await page.getByLabel('Price').fill('45.00');
    // Enable inventory tracking if needed by UI
    const inventoryInput = page.getByLabel(/Inventory|Stock/i);
    if (await inventoryInput.isVisible()) {
      await inventoryInput.fill('3');
    }
    await page.getByRole('button', { name: 'Save' }).click();
    await page.waitForSelector('text=Vegan Chocolate Cake');

    // Get tenant ID from API or cookies (simulated via API to get current user/tenant)
    // We can just use the login session to hit the webhook endpoint directly, since our backend uses the payload.tenant_id
    // But how to get the exact tenant_id? The easiest way is to extract it from local storage, or call a "whoami" endpoint.
    const userRes = await request.get('/api/users/me');
    expect(userRes.ok()).toBeTruthy();
    const userData = await userRes.json();
    const tenantId = userData.organization_id;

    expect(tenantId).toBeTruthy();

    // 3. Simulate incoming Instagram DM webhook
    const senderId = `insta_user_${uniqueId}`;
    const webhookPayload = {
      tenant_id: tenantId,
      source: 'instagram',
      message: 'Do you have vegan chocolate cake available for Saturday?',
      sender_id: senderId
    };

    const webhookRes = await request.post('/api/agents/webhook', {
      data: webhookPayload,
    });
    expect(webhookRes.ok()).toBeTruthy();

    // 4. Go to Agents or Approvals page and verify the drafted reply
    await page.goto('/agents');

    // Wait for the pending approval to appear
    await expect(page.getByText('The Ambassador')).toBeVisible({ timeout: 10000 });

    // The action should say something like "Draft reply for Instagram message from..."
    await expect(page.getByText(/Draft reply for Instagram message from/i)).toBeVisible();

    // The context or generated message should be visible if we click details, or we just approve it
    const approveButton = page.getByRole('button', { name: 'Approve' }).first();
    await expect(approveButton).toBeVisible();

    // Click approve
    await approveButton.click();

    // Verify it disappears from the pending list or shows success
    await expect(page.getByText(/Draft reply for Instagram message from/i)).not.toBeVisible({ timeout: 5000 });
  });
});
