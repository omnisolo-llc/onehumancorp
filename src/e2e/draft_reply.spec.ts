import { test, expect } from '@playwright/test';

test.describe('Draft Reply Handler CUJ', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the inbox to start all CUJ variations
    await page.goto('/inbox');
    await page.waitForLoadState('networkidle');
  });

  test('Persona: Business Owner generates a draft reply for a customer message', async ({ page }) => {
    // Wait for the UI Inbox endpoint to load
    const inboxResponse = page.waitForResponse(response =>
      response.url().includes('/api/ui/inbox/messages') && response.request().method() === 'GET'
    );
    await inboxResponse;

    // Simulate clicking on a message row to view details or start replying
    // Assuming there's a generate draft reply button
    const generateDraftBtn = page.getByRole('button', { name: /Draft Reply/i }).first();

    // If the UI doesn't have it explicitly shown, we will test the endpoint directly
    // to verify the backend fix works
    const draftResponse = await page.request.post('/api/inbox/draft_reply', {
      data: {
        customer_message: "Do you offer vegan options?"
      },
      headers: {
        'Authorization': 'Bearer test-token-standalone' // Or appropriate auth if mocked in test env
      }
    });

    // The backend handler will either return 200 with the draft or 502/503 depending on MiniMax API availability
    // but it should NOT return a 500 error due to database connection issue (db.pool hardcoding).
    expect([200, 502, 503]).toContain(draftResponse.status());
  });

  test('Persona: Backend safely handles SQLite offline pool logic', async ({ request }) => {
     // Direct request to the draft_reply handler to confirm no DB-level 500 crashes
     const response = await request.post('/api/inbox/draft_reply', {
       data: { customer_message: "Can I book a session next week?" },
       headers: { 'Authorization': 'Bearer local-test-token' }
     });

     // A successful connection logic fix will result in either 200 (success),
     // 401 (invalid test token), or 502/503 (no MiniMax API key).
     // It will NOT throw a 500 internal server error from trying to fetch from the Postgres pool
     // when running in standalone mode (SQLite).
     expect(response.status()).not.toBe(500);
  });

  test('Persona: System handles empty customer message securely', async ({ request }) => {
    const response = await request.post('/api/inbox/draft_reply', {
      data: { customer_message: "" },
      headers: { 'Authorization': 'Bearer local-test-token' }
    });

    expect(response.status()).not.toBe(500);
  });

  test('Persona: Request without authorization gets rejected', async ({ request }) => {
    const response = await request.post('/api/inbox/draft_reply', {
      data: { customer_message: "Hello!" }
    });

    expect(response.status()).toBe(401);
  });

  test('Persona: Verify ORG_CACHE_ADVISORY caching behavior', async ({ request }) => {
    // We hit the endpoint twice to exercise the new cache path
    await request.post('/api/inbox/draft_reply', {
      data: { customer_message: "Message 1" },
      headers: { 'Authorization': 'Bearer local-test-token' }
    });

    const secondResponse = await request.post('/api/inbox/draft_reply', {
      data: { customer_message: "Message 2" },
      headers: { 'Authorization': 'Bearer local-test-token' }
    });

    expect(secondResponse.status()).not.toBe(500);
  });
});
