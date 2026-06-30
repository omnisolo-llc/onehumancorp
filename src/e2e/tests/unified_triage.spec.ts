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
    await expect(page.locator('#unified-agent-feed-section')).toBeVisible();

    // Verify the newly seeded card is visible
    const triageCard = page.locator('.triage-item', { hasText: 'Instagram DM' }).first();
    await expect(triageCard).toBeVisible({ timeout: 10000 });

    // Check layout and glassmorphism styling
    await expect(triageCard).toHaveCSS('border-radius', '16px');
    await expect(triageCard).toHaveCSS('padding', '16px');
    await expect(page.locator('.triage-item').first()).toHaveCSS('backdrop-filter', 'blur(30px) saturate(210%)');

    // 4. Approve the drafted response
    const approveBtn = triageCard.getByTestId(/triage-review-/);
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();
    const confirmBtn = page.getByTestId('bottom-sheet-approve');
    await expect(confirmBtn).toBeVisible();
    await confirmBtn.click();

    // 5. Verify the item is marked as approved and visually dismissed/dimmed
    await expect(triageCard).not.toBeVisible();
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
    await expect(triageCard).not.toBeVisible();
  });

  test('Triage Feed displays an empty state beautifully', async ({ page }) => {
     const emptyTenantId = 'e2e-empty-tenant-' + Date.now();
     await page.addInitScript((t) => {
      window.localStorage.setItem('tenant_id', t);
    }, emptyTenantId);

    await page.goto('/api/ui/dashboard.html');

    const emptyMessage = page.locator('.triage-card.empty', { hasText: 'No recent activity found' });
    await expect(emptyMessage).toBeVisible();
  });

  test('Triage Feed properly links a customer context', async ({ page }) => {
     const customTenant = 'e2e-triage-customer-tenant';
     await page.addInitScript((t) => {
      window.localStorage.setItem('tenant_id', t);
    }, customTenant);

    const res = await page.request.post('/api/v1/omnichannel/webhook', {
        data: {
            tenant_id: customTenant,
            source: 'Email',
            sender_id: 'maya@example.com',
            message: 'I want to order a custom cake for my wedding',
        }
    });
    expect(res.status()).toBe(200);

    await new Promise(resolve => setTimeout(resolve, 5000));
    await page.goto('/api/ui/dashboard.html');

    const triageCard = page.locator('.triage-item', { hasText: 'Email' }).first();
    await expect(triageCard).toBeVisible({ timeout: 10000 });

    const contextText = await triageCard.locator('.triage-context').textContent();
    expect(contextText).toContain('inquiry'); // LLM might generate varying summaries, but it will have context.
  });

  test('Triage Feed proposes Draft Booking action correctly', async ({ page }) => {
    const bookingTenant = 'e2e-triage-booking-tenant';
    await page.addInitScript((t) => {
     window.localStorage.setItem('tenant_id', t);
   }, bookingTenant);

   // The LLM mock will try to determine action type. We simulate a booking request.
   const res = await page.request.post('/api/v1/omnichannel/webhook', {
       data: {
           tenant_id: bookingTenant,
           source: 'WhatsApp',
           sender_id: 'user_book_1',
           message: 'I would like to schedule an appointment for next Monday at 10am',
       }
   });
   expect(res.status()).toBe(200);

   await new Promise(resolve => setTimeout(resolve, 5000));
   await page.goto('/api/ui/dashboard.html');

   const triageCard = page.locator('.triage-item', { hasText: 'WhatsApp' }).first();
   await expect(triageCard).toBeVisible({ timeout: 10000 });

   // Verify action button exists
   const actionBtn = triageCard.getByRole('button', { name: /Approve|Send|Review/i });
   await expect(actionBtn).toBeVisible();
 });

});
