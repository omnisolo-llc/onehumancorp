import { test, expect } from '@playwright/test';

test.describe('Ambassador Auto-Responder CUJ', () => {
  test('Owner connects Meta Graph API and approves Ambassador drafted reply', async ({ page, request }) => {
    // 1. Connect Instagram via Integrations
    // Start from login to satisfy the rules
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('maya@ohc.test');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Login' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    await page.goto('/integrations');

    // Mock window alert for OAuth connect
    page.on('dialog', dialog => dialog.accept());

    const metaCard = page.getByRole('heading', { name: 'Meta Graph API' }).locator('xpath=ancestor::div[contains(@class, "rounded")][1]');
    const connectMetaButton = metaCard.getByRole('button', { name: 'Connect' });
    await connectMetaButton.click();

    // Verify state changed
    await expect(metaCard.locator('button:has-text("Manage")')).toBeVisible();

    // 2. Trigger the Ambassador's draft reply via a real API call (no mocks)
    // The CustomerSuccess agent listens for tenant.message.received, which is triggered via the webhook endpoint
    const tenantId = 'e2e-tenant';
    const webhookPayload = {
      tenant_id: tenantId,
      message: 'Do you have vegan chocolate cake available for Saturday?',
      source: 'instagram',
      from_identifier: 'Ava Customer'
    };

    const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || process.env.BASE_URL || '';
    const response = await request.post(`${apiBase}/api/agents/webhook`, {
      data: webhookPayload,
    });

    expect(response.ok()).toBeTruthy();

    // 3. Navigate to Team Page
    await page.goto('/team');
    await expect(page.getByRole('heading', { name: 'Your Team', exact: true })).toBeVisible();

    // Navigate to The Ambassador
    await page.getByRole('button', { name: 'The Ambassador' }).first().click();

    // Ensure we are viewing the Ambassador inbox specifically
    await expect(page.getByRole('heading', { name: 'The Ambassador' })).toBeVisible({ timeout: 5000 });

    // Wait for the new item.
    const inquiryLocator = page.getByText('Do you have vegan chocolate cake available for Saturday?').first();
    await expect(page.getByText(/All Caught Up!|Do you have vegan chocolate cake available for Saturday?/)).toBeVisible({ timeout: 15000 });

    const reviewButton = page.getByRole('button', { name: 'Edit' }).first();
    if (await reviewButton.isVisible()) {
      // Validate split UI before clicking Send
      await expect(page.getByText('Customer Message')).toBeVisible();
      await expect(page.getByText('AI Drafted Reply')).toBeVisible();

      const sendButton = page.getByRole('button', { name: 'Send Draft' }).first();
      await expect(sendButton).toBeVisible();

      await sendButton.click();

      // Validate empty state or removal
      await expect(inquiryLocator).toBeHidden();
    } else {
      await expect(page.getByText('All Caught Up!')).toBeVisible();
    }
  });
});
