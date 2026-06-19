import { test, expect } from '@playwright/test';

test.describe('Unified Multi-Channel Work Triage & AI Inbox Engine', () => {
  const tenantId = 'e2e-triage-tenant';

  test.beforeAll(async ({ request }) => {
    // 1. Seed a message via webhook to simulate ingestion
    // Webhook ingestion creates an `ohc_job_queue` record which the local background worker processes.
    // The background worker uses Minimax mock returning deterministic triage data.
    const res = await request.post('/api/v1/omnichannel/webhook', {
      data: {
        tenant_id: tenantId,
        source: 'Instagram DM',
        sender_id: 'carlos_handyman',
        message: 'Can you fix my sink tomorrow?',
      }
    });

    expect(res.status()).toBe(200);

    // Wait for the background worker to process the message and insert into agent_feed_items/triage_items
    // Since this can be asynchronous, we'll give it a moment or rely on UI polling.
    await new Promise(resolve => setTimeout(resolve, 5000));
  });

  test('User can view triage feed and approve drafted replies', async ({ page }) => {
    // Mock the localStorage tenant
    await page.addInitScript((t) => {
      window.localStorage.setItem('tenant_id', t);
    }, tenantId);

    // 2. Load the Dashboard UI where Triage Feed is rendered
    await page.goto('/api/ui/dashboard.html');

    // 3. Verify Triage cards are rendered
    await expect(page.locator('#triage-queue')).toBeVisible();

    // Verify the newly seeded card is visible
    const triageCard = page.locator('.triage-item', { hasText: 'Instagram DM' }).first();
    await expect(triageCard).toBeVisible({ timeout: 10000 });

    // Check layout and glassmorphism styling
    await expect(triageCard).toHaveCSS('border-radius', '12px');
    await expect(triageCard).toHaveCSS('padding', '20px');
    await expect(page.locator('.triage-card')).toHaveCSS('backdrop-filter', 'blur(16px)');

    // 4. Approve the drafted response
    const approveBtn = triageCard.getByTestId('approve-btn');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // 5. Verify the item is marked as approved and visually dismissed/dimmed
    await expect(triageCard).toHaveCSS('opacity', '0.5');
  });

  test('User can dismiss a triage item', async ({ page }) => {
    // Mock the localStorage tenant
    await page.addInitScript((t) => {
      window.localStorage.setItem('tenant_id', t);
    }, tenantId);

    // Seed another item using the webhook
    await page.request.post('/api/v1/omnichannel/webhook', {
        data: {
            tenant_id: tenantId,
            source: 'WhatsApp',
            sender_id: 'cust-456',
            message: 'Inquiry about pricing',
        }
    });

    await new Promise(resolve => setTimeout(resolve, 5000));

    await page.goto('/api/ui/dashboard.html');

    const triageCard = page.locator('.triage-item', { hasText: 'WhatsApp' }).first();
    await expect(triageCard).toBeVisible({ timeout: 10000 });

    const dismissBtn = triageCard.getByRole('button', { name: 'Dismiss' });
    await expect(dismissBtn).toBeVisible();
    await dismissBtn.click();

    // Verify it is dimmed
    await expect(triageCard).toHaveCSS('opacity', '0.5');
  });

  test('Triage Feed displays an empty state beautifully', async ({ page }) => {
     const emptyTenantId = 'e2e-empty-tenant-' + Date.now();
     await page.addInitScript((t) => {
      window.localStorage.setItem('tenant_id', t);
    }, emptyTenantId);

    await page.goto('/api/ui/dashboard.html');

    const emptyMessage = page.locator('.triage-card.empty', { hasText: 'No items need your attention right now' });
    await expect(emptyMessage).toBeVisible();
  });
});
