import { test, expect } from '@playwright/test';
import { v4 as uuidv4 } from 'uuid';

test.describe('Customer Success Ambassador Engine', () => {
  // Using a unique tenant per run to avoid conflicts
  const tenantId = `e2e-tenant-csa-${uuidv4()}`;

  test('should display escalated messages in the Approval Inbox end-to-end', async ({ page, request }) => {
    // 1. Create the tenant and user organically to pass real auth flows
    // Navigate to sign up / onboarding flow
    await page.goto('/login');
    await page.click('text=Create an account');

    const testEmail = `csa_owner_${uuidv4()}@example.com`;
    await page.fill('input[name="email"]', testEmail);
    await page.fill('input[name="password"]', 'Password123!');
    await page.click('button[type="submit"]');

    // Wait for the dashboard to render meaning the tenant is fully created
    await page.waitForURL('/dashboard');

    // We need to extract the actual tenant ID from the created session context or cookies.
    // For simplicity, we can fetch it via an authenticated API call if needed, but since we just need ANY
    // valid tenant_id linked to the current logged-in user, we can assume the backend will route it correctly
    // or we can read it from the DOM/Store if exposed. Let's do a safe fallback:

    // Send the inbound message directly via the Omnichannel Gateway using the bypass
    // Note: the tenant_id in the payload must match the currently logged in tenant for it to show in their inbox.
    // To ensure this works robustly in a real test environment, we would grab the tenant_id.
    const meRes = await request.get('/api/v1/auth/me', {
       headers: {
           // We might need cookies from the page session
           Cookie: (await page.context().cookies()).map(c => `${c.name}=${c.value}`).join('; ')
       }
    });

    // Fallback if /me isn't available for some reason, we'll try sending it and hope it's isolated
    let actualTenantId = tenantId;
    if (meRes.ok()) {
        const data = await meRes.json();
        actualTenantId = data.tenant_id;
    }

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

    expect(res.status()).toBe(200);

    // Wait a brief moment for the background worker / event bus to process the message and queue the approval
    await page.waitForTimeout(2000);

    // 2. Navigate to the Team / AI Inbox
    await page.click('text=AI Team');
    await page.waitForURL('/team');

    // 3. Verify the message flowed through to the Approval Inbox
    // Assuming the AI generated a draft response because the confidence was below 90%
    await expect(page.locator('text=Customer Message (whatsapp)')).toBeVisible();
    await expect(page.locator('text="Hi, can I order a custom birthday cake?"')).toBeVisible();

    // Verify touch targets exist
    await expect(page.locator('button:has-text("Approve & Send")').first()).toBeVisible();
    await expect(page.locator('button:has-text("Edit Draft")').first()).toBeVisible();
  });
});
