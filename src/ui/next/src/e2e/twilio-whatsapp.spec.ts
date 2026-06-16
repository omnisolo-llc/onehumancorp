import { test, expect } from '@playwright/test';

test.describe('Twilio WhatsApp Flow CUJ', () => {
  test('Owner connects Twilio WhatsApp via Integrations', async ({ page, request }) => {
    test.setTimeout(300000);

    // 1. Log in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('maya@ohc.test');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();
    await expect(page.getByRole('heading', { name: /Dashboard/i }).first()).toBeVisible({ timeout: 30000 });

    // 2. Connect Twilio
    await page.goto('/integrations');
    const twilioCard = page.locator('h3', { hasText: 'Twilio Conversations' }).locator('..');
    await twilioCard.getByRole('button', { name: /Connect/i }).click();

    // 3. Mock the Twilio signup flow
    await expect(page.getByRole('heading', { name: /Connect Twilio Conversations/i })).toBeVisible();
    await page.getByRole('button', { name: /Save & Connect/i }).click();

    // After connecting, it should redirect to inbox
    await expect(page).toHaveURL(/\/inbox/);

    // 4. Trigger inbound message via webhook
    const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || 'http://localhost:18789';

    // Send standard application/x-www-form-urlencoded
    const response = await request.post(`${apiBase}/api/v1/webhooks/twilio`, {
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded',
      },
      data: 'From=whatsapp%3A%2B14155238886&To=whatsapp%3A%2B14155238886&Body=Hello!%20I%20like%20to%20order%20a%20twilio%20vegan%20cake',
    });
    expect(response.ok()).toBeTruthy();

    // 5. Check that the WhatsApp message text appears
    await page.goto('/inbox');
    await expect(page.getByText(/Hello! I like to order a twilio vegan cake/i).first()).toBeVisible({ timeout: 15000 });
  });
});
