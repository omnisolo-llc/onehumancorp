import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Mobile Payload Optimization', () => {

  test('UI Triage request succeeds with mobile_optimized=true', async ({ adminPage, request }) => {
     // Verify the actual endpoint returns 200 OK and valid JSON
     const response = await request.get(`/api/ui/triage?mobile_optimized=true`);
     expect(response.status()).toBe(200);
     const json = await response.json();
     expect(Array.isArray(json)).toBeTruthy();
     if (json.length > 0) {
         expect(json[0].context).toBeUndefined();
         expect(json[0].action_payload).toBeUndefined();
     }
  });

  test('UI Triage request succeeds with mobile_optimized=false', async ({ adminPage, request }) => {
     const response = await request.get(`/api/ui/triage?mobile_optimized=false`);
     expect(response.status()).toBe(200);
     const json = await response.json();
     expect(Array.isArray(json)).toBeTruthy();
  });

  test('UI Inbox request succeeds with mobile_optimized=true', async ({ adminPage, request }) => {
     const response = await request.get(`/api/ui/inbox?mobile_optimized=true`);
     expect(response.status()).toBe(200);
     const json = await response.json();
     expect(Array.isArray(json)).toBeTruthy();
     if (json.length > 0) {
         expect(json[0].original_message).toBeUndefined();
         expect(json[0].generated_response).toBeUndefined();
     }
  });

  test('UI Inbox request succeeds with mobile_optimized=false', async ({ adminPage, request }) => {
     const response = await request.get(`/api/ui/inbox?mobile_optimized=false`);
     expect(response.status()).toBe(200);
     const json = await response.json();
     expect(Array.isArray(json)).toBeTruthy();
  });

  test('UI Unified Feed request succeeds with mobile_optimized=true', async ({ adminPage, request }) => {
     const response = await request.get(`/api/ui/dashboard/unified-feed?mobile_optimized=true`);
     expect(response.status()).toBe(200);
     const json = await response.json();
     expect(json.triage).toBeDefined();
     expect(json.inbox).toBeDefined();
  });
});
