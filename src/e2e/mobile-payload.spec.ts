
import { test, expect } from './fixtures';


test.describe('Mobile Payload Optimization', () => {

  let headers = {};

  test.beforeAll(async () => {
    const crypto = require('crypto');
    function base64url(source) {
      return Buffer.from(source).toString('base64')
        .replace(/=/g, '').replace(/\+/g, '-').replace(/\//g, '_');
    }
    const header = { "alg": "none", "typ": "JWT" };
    const payload = { "sub": "test", "organization_id": "test_tenant" };
    const token = base64url(JSON.stringify(header)) + "." + base64url(JSON.stringify(payload)) + ".";

    headers = {
        'Authorization': 'Bearer ' + token,
        'X-Tenant-ID': 'test_tenant'
    };
  });

  test('UI Triage request succeeds with mobile_optimized=true and does not include context or action_payload', async ({ request }) => {
     // Verify the actual endpoint returns 200 OK and valid JSON
     const response = await request.get(`/api/ui/triage?mobile_optimized=true`, { headers });
     expect(response.status()).toBe(200);
     const json = await response.json();
     expect(Array.isArray(json)).toBeTruthy();
     if (json.length > 0) {
         expect(json[0].context).toBeUndefined();
         expect(json[0].action_payload).toBeUndefined();
         if (json[0].intent) {
            expect(json[0].customer_info).toBeUndefined();
            expect(json[0].suggested_actions).toBeUndefined();
         }
     }
  });

  test('UI Triage request succeeds with mobile_optimized=false', async ({ request }) => {
     const response = await request.get(`/api/ui/triage?mobile_optimized=false`, { headers });
     expect(response.status()).toBe(200);
     const json = await response.json();
     expect(Array.isArray(json)).toBeTruthy();
  });

  test('UI Inbox request succeeds with mobile_optimized=true and does not include original_message', async ({ request }) => {
     const response = await request.get(`/api/ui/inbox/messages?mobile_optimized=true`, { headers });
     expect(response.status()).toBe(200);
     const json = await response.json();
     expect(Array.isArray(json)).toBeTruthy();
     if (json.length > 0) {
         expect(json[0].original_message).toBeUndefined();
         expect(json[0].generated_response).toBeUndefined();
     }
  });

  test('UI Inbox request succeeds with mobile_optimized=false', async ({ request }) => {
     const response = await request.get(`/api/ui/inbox/messages?mobile_optimized=false`, { headers });
     expect(response.status()).toBe(200);
     const json = await response.json();
     expect(Array.isArray(json)).toBeTruthy();
  });

  test('UI Unified Feed request succeeds with mobile_optimized=true', async ({ request }) => {
     const response = await request.get(`/api/ui/dashboard/unified-feed?mobile_optimized=true`, { headers });
     expect(response.status()).toBe(200);
     const json = await response.json();
     expect(json.triage).toBeDefined();
     expect(json.inbox).toBeDefined();
  });


  test('UI Orders request succeeds with mobile_optimized=true', async ({ request }) => {
     const response = await request.get(`/api/ui/orders?mobile_optimized=true`, { headers });
     expect(response.status()).toBe(200);
     const json = await response.json();
     expect(Array.isArray(json)).toBeTruthy();
     if (json.length > 0) {
         expect(json[0].customer_name).toBeUndefined();
     }
  });

  test('UI Bookings request succeeds with mobile_optimized=true', async ({ request }) => {
     const response = await request.get(`/api/ui/bookings?mobile_optimized=true`, { headers });
     expect(response.status()).toBe(200);
     const json = await response.json();
     expect(Array.isArray(json)).toBeTruthy();
     if (json.length > 0) {
         expect(json[0].customer_name).toBeUndefined();
         expect(json[0].end_time).toBeUndefined();
     }
  });

  test('UI Supply request succeeds with mobile_optimized=true', async ({ request }) => {
     const response = await request.get(`/api/ui/supply?mobile_optimized=true`, { headers });
     expect(response.status()).toBe(200);
     const json = await response.json();
     expect(json.vendors).toBeDefined();
     expect(json.raw_materials).toBeDefined();
     expect(json.bom_items).toBeDefined();
  });

  test('UI Omni Inbox request succeeds with mobile_optimized=true', async ({ request }) => {
     const response = await request.get(`/api/ui/omni_inbox?mobile_optimized=true`, { headers });
     expect(response.status()).toBe(200);
     const json = await response.json();
     expect(Array.isArray(json)).toBeTruthy();
     if (json.length > 0) {
         expect(json[0].original_content).toBeUndefined();
         expect(json[0].draft_reply).toBeUndefined();
     }
  });
});
