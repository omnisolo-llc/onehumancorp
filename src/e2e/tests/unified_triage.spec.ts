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

test.describe('Work Triage Engine deduplication and prioritization', () => {
  const tenantId = 'e2e-triage-engine-tenant';

  test('groups low stock alerts and prioritizes deposit failures via full UI flow', async ({ page }) => {
    // Navigate to the test login page and use the specific test user credentials
    await page.goto('/login');
    await page.fill('input[name="email"]', 'owner@e2e-triage-engine.com');
    await page.fill('input[name="password"]', 'testpassword123');
    await page.click('button[type="submit"]');

    // Wait for the dashboard to load indicating successful login
    // await expect(page.locator('#triage-queue')).toBeVisible({ timeout: 15000 });
    // Assuming UI lands on dashboard with feed items

    await page.goto('/dashboard');
    // Verify deposit failed is the first item due to priority
    const items = page.locator('.bg-\\[rgba\\(255\\,255\\,255\\,0\\.65\\)\\]'); // the card class from UI

    // We expect 2 distinct items based on the SQL seed for this user
    await expect(items).toHaveCount(2, { timeout: 10000 });

    const firstItem = items.nth(0);
    await expect(firstItem).toContainText('deposit failed', { ignoreCase: true });
    await expect(firstItem).toContainText('Urgent Action Required', { ignoreCase: true });

    const secondItem = items.nth(1);
    await expect(secondItem).toContainText('low stock', { ignoreCase: true });
    await expect(secondItem).toContainText('3 items'); // Deduplicated count
  });
});
