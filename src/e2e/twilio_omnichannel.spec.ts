import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('twilio_omnichannel');

import { test, expect } from '@playwright/test';
import { seedTenantWithEmail } from './helpers/seed';

test.describe('Twilio Webhook & Triage Integration', () => {
  let tenantId: string;

  test.beforeEach(async () => {
    const seedResult = await seedTenantWithEmail();
    tenantId = seedResult.tenant_id;
  });

  test('Receives WhatsApp message and displays in Triage', async ({ request, page }) => {
    const phone = '+1234567890';
    const messageText = 'Hello from WhatsApp ' + Date.now();

    // Simulate Twilio webhook request
    const res = await request.post('/api/v1/webhooks/twilio', {
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded',
      },
      data: \`From=\${encodeURIComponent(phone)}&To=%2B0987654321&Body=\${encodeURIComponent(messageText)}\`
    });

    expect(res.status()).toBe(200);

    // Setup login context via cookie
    await page.context().addCookies([
      {
        name: 'ohc_session',
        value: 'ohc_fake_token',
        domain: 'localhost',
        path: '/',
      },
      {
        name: 'ohc_tenant',
        value: tenantId,
        domain: 'localhost',
        path: '/',
      }
    ]);

    await page.goto('/triage');

    // Wait for the message to appear in the Triage UI
    const sourceLabel = page.locator('.app-list-title', { hasText: 'whatsapp' }).first();
    await expect(sourceLabel).toBeVisible();
  });
});
