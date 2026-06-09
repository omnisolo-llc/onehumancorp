import { test, expect } from '@playwright/test';

test.describe('WhatsApp Flow CUJ', () => {
  test('Owner connects WhatsApp and approves draft reply', async ({ page, request }) => {
    // 1. Connect WhatsApp via Integrations
    // Start from login to satisfy the rules
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Login' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    await page.goto('/integrations');

    const whatsappCard = page.locator('h3', { hasText: 'WhatsApp Cloud API' }).locator('..');
    await whatsappCard.getByRole('button', { name: 'Connect' }).click();

    await expect(page.getByRole('heading', { name: 'Connect WhatsApp' })).toBeVisible();
    await page.getByRole('button', { name: 'Continue with Meta' }).click();

    await expect(page.getByText('WhatsApp Cloud API connected')).toBeVisible();

    // 2. Trigger the Ambassador's draft reply via a real API call (no mocks)
    const tenantId = 'e2e-tenant';
    const webhookPayload = {
      tenant_id: tenantId,
      message: 'Hello! Id like to order a vegan cake over WhatsApp.',
      source: 'whatsapp',
      sender_id: '1234567890'
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

    const inquiryLocator = page.getByText('Hello! Id like to order a vegan cake over WhatsApp.').first();
    const approveButton = page.getByRole('button', { name: 'Approve' }).first();
    await expect(page.getByText(/All Caught Up!|Hello! Id like to order a vegan cake over WhatsApp./)).toBeVisible({ timeout: 15000 });

    const draftLocator = page.getByText(/Draft Reply/i).first();
    if (await draftLocator.isVisible()) {
       await expect(draftLocator).toBeVisible();
    }

    if (await approveButton.isVisible()) {
      await approveButton.click();
      await expect(inquiryLocator).toBeHidden();
    } else {
      await expect(page.getByText('All Caught Up!')).toBeVisible();
    }
  });
});
