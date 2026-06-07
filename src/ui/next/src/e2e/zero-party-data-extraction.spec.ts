import { test, expect } from '@playwright/test';

test.describe('Zero-Party Data Extraction CUJ', () => {
  test('Ambassador agent automatically extracts preferences from incoming message and updates customer profile', async ({ page, request }) => {
    // 1. Log in first so that the test creates data in the tenant associated with the logged-in user
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('maya@ohc.test');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Login' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // 2. We extract the actual tenant_id from local storage that was populated during login
    const tenantId = await page.evaluate(() => localStorage.getItem('ohc_tenant_id') || 'default');

    // 3. Simulate an incoming DM via webhook to the user's real tenant
    const customerId = 'celiac_customer';
    const webhookPayload = {
      tenant_id: tenantId,
      message: 'Hi, do you have any gluten-free options? I have celiac.',
      source: 'instagram',
      sender_id: customerId
    };

    const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || process.env.BASE_URL || 'http://127.0.0.1:18789';
    const response = await request.post(`${apiBase}/api/agents/webhook`, {
      data: webhookPayload,
    });

    expect(response.ok()).toBeTruthy();

    // 4. Wait to ensure processing completes
    await page.waitForTimeout(3000);

    // 5. Navigate to Customers view
    await page.goto('/customers');
    await expect(page.getByRole('heading', { name: 'Customers' })).toBeVisible();

    // Wait for the specific customer to appear
    const customerLink = page.getByText(customerId);
    await expect(customerLink).toBeVisible({ timeout: 10000 });

    // Click into customer detail
    await customerLink.click();

    // 5. Verify the profile view has the extracted preferences
    await expect(page.getByRole('heading', { name: customerId })).toBeVisible();
    await expect(page.getByTestId('customer-preferences')).toBeVisible();

    // 6. Check Agent Feed in Dashboard
    await page.goto('/dashboard');
    await page.getByRole('tab', { name: 'Activity' }).click();

    // Validate we see the feed event
    await expect(page.getByText('The Ambassador learned that customer celiac_customer has preferences')).toBeVisible({ timeout: 10000 });
  });
});
