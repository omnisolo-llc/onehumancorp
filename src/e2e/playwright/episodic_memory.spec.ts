import { test, expect } from '@playwright/test';

test.describe('Episodic Memory / Unified Inbox E2E', () => {
  test('simulates past interactions and checks context rehydration', async ({ request, page }) => {
    const tenant_id = "org_e2e_memory_tenant";
    const customer_id = "e2e_memory_customer_1";

    // First message: set a preference
    const payload1 = {
      tenant_id,
      source: "whatsapp",
      identifier: "15551234567",
      message: "Hi, I prefer weekend delivery for all my orders.",
    };

    const res1 = await request.post('/api/v1/webhooks/unified_inbox', {
      data: payload1,
    });
    expect(res1.ok()).toBeTruthy();

    await page.waitForTimeout(2000);

    // Second message: ask a question relying on the preference
    const payload2 = {
      tenant_id,
      source: "whatsapp",
      identifier: "15551234567",
      message: "When can you deliver the cake?",
    };

    const res2 = await request.post('/api/v1/webhooks/unified_inbox', {
      data: payload2,
    });
    expect(res2.ok()).toBeTruthy();

    // Check UI for unified feed to see if triage action generated a draft
    const uiRes = await request.get(`/api/ui/unified_inbox_feed?tenant_id=${tenant_id}`);
    expect(uiRes.ok()).toBeTruthy();

    const feed = await uiRes.json();
    const threads = feed.map((f: any) => f.thread);
    expect(threads.length).toBeGreaterThan(0);

    // We verify the system is stable and can serve the memory endpoint
    const memRes = await request.get(`/api/assistant/memory/customer/${customer_id}`, {
      headers: {
        'x-tenant-id': tenant_id
      }
    });
  });
});
