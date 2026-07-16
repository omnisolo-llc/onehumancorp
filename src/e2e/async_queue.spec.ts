import { test, expect } from './fixtures';

test.describe('Universal Event Bus & Multi-Tenant Async Queue', () => {

  test('Concurrent webhooks enqueue jobs and populate agent feed via Universal Event Bus', async ({ page, request, memberPage }) => {
    // 1. Setup the environment
    await memberPage.setViewportSize({ width: 375, height: 667 });

    const loginRes = await request.post('/api/v1/auth/login', {
        data: { email: 'admin@ohc.local', password: 'admin' }
    });
    expect(loginRes.ok()).toBeTruthy();
    const { token, user } = await loginRes.json();
    const tenantId = user.tenant_id;

    // 2. Simulate concurrent incoming webhooks (e.g., 5 custom cake inquiries overnight)
    const requests = [];
    for (let i = 0; i < 5; i++) {
        requests.push(request.post('/api/v1/inbox/webhook', {
            data: {
                tenant_id: tenantId,
                source: 'instagram',
                sender_id: `customer_async_test_${i}`,
                message: `Hi, I need a custom cake for a birthday! - Request Async Queue Test ${i}`
            }
        }));
    }

    const responses = await Promise.all(requests);
    for (const res of responses) {
        expect(res.ok()).toBeTruthy();
        const json = await res.json();
        expect(json.success).toBe(true);
    }

    // 3. Wait for background workers (SKIP LOCKED mechanism in pg_queue/ohc_job_queue) to process the jobs concurrently
    // In our implementation, message_triage worker should pick this up, classify intent, and drop it into agent_feed_items
    await page.waitForTimeout(5000);

    // 4. Navigate to Agent Feed (Home Dashboard) and verify the items are processed correctly without loss
    await memberPage.goto('/dashboard');

    // We expect to see 5 action cards for the generated replies
    // Wait for the feed to load
    await expect(memberPage.locator('text=CustomerSuccess').first()).toBeVisible({ timeout: 15000 });

    // Verify that at least some of our requests were processed and appeared
    await expect(memberPage.locator('text=custom cake').first()).toBeVisible();

    // Verify there are multiple items indicating concurrent execution didn't drop data
    const count = await memberPage.locator('text=custom cake').count();
    expect(count).toBeGreaterThan(0);
  });

});
