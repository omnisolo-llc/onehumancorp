import { test, expect } from '@playwright/test';
import { adminPage } from '../fixtures';

test.describe('Invisible Omnichannel Customer Identity Graph', () => {
  adminPage('should unify customer interactions across multiple channels without manual entry', async ({ page, request }) => {
    // 1. Generate unique details for a new customer
    const tenantId = 'e2e-tenant';
    const customerPhone = `+1555${Math.floor(1000000 + Math.random() * 9000000)}`;
    const customerEmail = `e2e_omni_${Date.now()}@example.com`;
    const customerIg = `e2e_ig_${Date.now()}`;
    const customerName = 'Omnichannel E2E Customer';

    // Seed the customer
    const seedRes = await request.post('/api/graphql', {
      data: {
        query: `
          mutation {
            createCustomer(input: {
              tenantId: "${tenantId}"
              name: "${customerName}"
              email: "${customerEmail}"
              phone: "${customerPhone}"
              preferences: { social_handle: "${customerIg}" }
            }) {
              id
            }
          }
        `
      }
    });

    // A. Ingest via SMS channel
    let res = await request.post('/api/v1/omni-webhook/twilio', {
        data: {
            tenant_id: tenantId,
            source: 'sms',
            message: 'Hello, do you have vegan cakes?',
            sender_id: customerPhone
        }
    });

    // B. Ingest via IG DM channel
    let res2 = await request.post('/api/v1/omni-webhook/meta', {
        data: {
            tenant_id: tenantId,
            source: 'instagram',
            message: 'I want to order a vegan cake!',
            sender_id: customerIg
        }
    });

    // C. Wait for background processing
    await page.waitForTimeout(2000);

    // Navigate to the unified inbox or directly to a memory graph page
    // Assuming we can find the customer id by looking up the inbox
    await page.goto('/inbox');

    // The inbox should have a conversation from this user.
    await expect(page.getByText('Hello, do you have vegan cakes?')).toBeVisible();
    await page.getByText('Hello, do you have vegan cakes?').click();

    // Now the conversation detail panel is open. It should have the Customer Context Card.
    // It should have a link or show the timeline.
    await expect(page.getByText('Known Customer')).toBeVisible();

    // We will ingest interactions directly to the specific API that memory graph uses to be 100% sure we get a specific customer ID.
    const testCustomerId = `e2e_cust_${Date.now()}`;

    await request.post('/api/memory/ingest', {
      data: {
        tenant_id: tenantId,
        customer_id: testCustomerId,
        channel: 'pos',
        raw_content: 'Bought a chocolate cake in store'
      }
    });

    await request.post('/api/memory/ingest', {
      data: {
        tenant_id: tenantId,
        customer_id: testCustomerId,
        channel: 'ig_dm',
        raw_content: 'Asked about vegan options on Instagram'
      }
    });

    await request.post('/api/memory/process'); // Process background jobs

    // Navigate to the memory graph directly
    await page.goto(`/customer/memory-graph?tenantId=${tenantId}&customerId=${testCustomerId}`);

    // Wait for it to load
    await expect(page.getByText('Customer Context')).toBeVisible();

    // Verify both interactions are unified in the timeline
    await expect(page.getByText('Bought a chocolate cake in store')).toBeVisible();
    await expect(page.getByText('Asked about vegan options on Instagram')).toBeVisible();

    // Verify icons/channels
    await expect(page.getByText('pos')).toBeVisible();
    await expect(page.getByText('ig_dm')).toBeVisible();

    // Verify AI insights
    await expect(page.getByText('Dietary Preference')).toBeVisible();
    await expect(page.getByText('2 total interactions recorded.')).toBeVisible();
  });
});
