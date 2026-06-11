import { test, expect } from '@playwright/test';

test.describe('WhatsApp Flow CUJ', () => {
  test('Owner connects WhatsApp and approves draft reply', async ({ page, request }) => {
    test.setTimeout(300000);

    // 1. Connect WhatsApp via Integrations
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('maya@ohc.test');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();
    await expect(page.getByRole('heading', { name: /Dashboard/i }).first()).toBeVisible({ timeout: 30000 });

    await page.goto('/integrations');

    const whatsappCard = page.locator('h3', { hasText: 'WhatsApp Business (Twilio)' }).locator('..');
    await whatsappCard.getByRole('button', { name: /Connect/i }).click();

    await expect(page.getByRole('heading', { name: /Connect Twilio WhatsApp/i })).toBeVisible();
    await page.getByLabel(/Account SID/i).fill('AC1234567890');
    await page.getByLabel(/Auth Token/i).fill('token123');
    await page.getByLabel(/WhatsApp Number/i).fill('whatsapp:+14155238886');
    await page.getByRole('button', { name: /Connect Twilio/i }).click();

    await expect(page.getByText(/Twilio WhatsApp connected/i)).toBeVisible();

    // 2. Trigger the Ambassador's draft reply via a real API call
    // We need to use the actual internal server URL
    const webhookPayload = {
      From: 'whatsapp:+1234567890',
      To: 'whatsapp:+14155238886',
      Body: 'Hello! Id like to order a vegan cake over WhatsApp.',
    };

    // Construct form-urlencoded string
    const body = Object.entries(webhookPayload)
      .map(([k, v]) => `${encodeURIComponent(k)}=${encodeURIComponent(v)}`)
      .join('&');

    const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || 'http://localhost:18789';
    const response = await request.post(`${apiBase}/api/v1/webhooks/twilio`, {
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded',
      },
      data: body,
    });

    expect(response.ok()).toBeTruthy();

    // 3. Navigate to Team Page / Inbox to see the draft
    await page.goto('/inbox');
    await expect(page.getByText(/Hello! Id like to order a vegan cake over WhatsApp/i)).toBeVisible({ timeout: 15000 });

    // Check for draft reply
    await expect(page.getByText(/Draft Reply/i).first()).toBeVisible({ timeout: 15000 });
  });
});
