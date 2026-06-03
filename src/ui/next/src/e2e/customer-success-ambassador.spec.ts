import { test, expect } from '@playwright/test';
import { v4 as uuidv4 } from 'uuid';

test.describe('Customer Success Ambassador Engine', () => {
  const tenantId = `e2e-tenant-csa-${uuidv4()}`;

  test('should display escalated messages in the Approval Inbox end-to-end', async ({ page, request }) => {
    // 1. Create the tenant and user organically to pass real auth flows
    await page.goto('/login');
    await page.click('text=Create an account');

    const testEmail = `csa_owner_${uuidv4()}@example.com`;
    await page.fill('input[name="email"]', testEmail);
    await page.fill('input[name="password"]', 'Password123!');
    await page.click('button[type="submit"]');

    await page.waitForURL('/dashboard');

    const meRes = await request.get('/api/v1/auth/me', {
       headers: {
           Cookie: (await page.context().cookies()).map(c => `${c.name}=${c.value}`).join('; ')
       }
    });

    let actualTenantId = tenantId;
    if (meRes.ok()) {
        const data = await meRes.json();
        actualTenantId = data.tenant_id;
    }

    // Fire directly into the new Omnichannel Gateway with the test bypass signature
    const res = await request.post('/api/v1/omnichannel/receive', {
      headers: {
        'X-Test-Bypass': 'true'
      },
      data: {
        tenant_id: actualTenantId,
        platform: 'whatsapp',
        original_message: 'Hi, can I order a custom birthday cake?',
        attachments: []
      }
    });

    // We expect the backend to accept it and process it
    expect(res.status()).toBe(200);

    // Wait for the background worker / event bus to process the message and queue the approval
    await page.waitForTimeout(4000);

    // 2. Navigate to the Team / AI Inbox
    await page.click('text=AI Team');
    await page.waitForURL('/team');

    // 3. Verify the message flowed through to the Approval Inbox
    await expect(page.locator('text=Customer Message (whatsapp)')).toBeVisible();
    await expect(page.locator('text="Hi, can I order a custom birthday cake?"')).toBeVisible();

    // Verify touch targets exist
    await expect(page.locator('button:has-text("Approve & Send")').first()).toBeVisible();
    await expect(page.locator('button:has-text("Edit Draft")').first()).toBeVisible();
  });
});
