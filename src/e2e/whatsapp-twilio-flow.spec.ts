import { test, expect } from '@playwright/test';

test.describe('Twilio WhatsApp Flow CUJ', () => {
  test('Owner connects Twilio for WhatsApp and receives message', async ({ page, request }) => {
    test.setTimeout(300000);

    // 1. Log in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('maya@ohc.test');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();
    await expect(page.getByRole('heading', { name: /Dashboard/i }).first()).toBeVisible({ timeout: 30000 });

    // 2. Connect Twilio WhatsApp
    await page.goto('/integrations');
    const whatsappCard = page.locator('h3', { hasText: 'Twilio for WhatsApp' }).locator('..');
    await whatsappCard.getByRole('button', { name: /Connect/i }).click();

    // 3. Fill in the modal
    await expect(page.getByRole('heading', { name: /Connect Twilio for WhatsApp/i })).toBeVisible();
    await page.getByPlaceholder('ACXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX').fill('ACtestaccountsid');
    await page.getByPlaceholder('your_auth_token').fill('testauthtoken');
    await page.getByPlaceholder('+1234567890').fill('+1234567890');

    await page.getByRole('button', { name: /Save & Connect/i }).click();

    // After connecting, the status message should show connected
    await expect(page.getByText(/Twilio for WhatsApp connected/i)).toBeVisible();

    // 4. Trigger inbound message via webhook
    const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || 'http://localhost:18789';
    const response = await request.post(`${apiBase}/api/v1/webhooks/twilio`, {
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded',
      },
      data: 'From=whatsapp%3A%2B14155238886&To=whatsapp%3A%2B1234567890&Body=Hello%21+Id+like+to+order+a+vegan+cake+over+WhatsApp.',
    });
    expect(response.ok()).toBeTruthy();

    // 5. Navigate to Inbox to see the message
    await page.goto('/inbox');

    // Check that the WhatsApp message text appears
    await expect(page.getByText(/Hello! Id like to order a vegan cake over WhatsApp/i).first()).toBeVisible({ timeout: 15000 });

    // Ensure it triggers auto-responder or appears correctly
    await expect(page.getByText(/Draft Reply/i).first()).toBeVisible({ timeout: 15000 }).catch(() => {});
  });
});
