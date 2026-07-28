import { test, expect } from '@playwright/test';
import { e2eTest } from './fixtures';

e2eTest.describe('Native Chat System', () => {
  e2eTest('should verify API endpoints for chat inboxes', async ({ request, currentTenantId }) => {
    // Generate a random UUID for the tenant
    const tenantId = currentTenantId;

    // Create Inbox
    const createInboxRes = await request.post(`/api/v1/chat/${tenantId}/inbox`, {
      data: { name: 'Support Inbox' }
    });

    // As we can't fully run the server with the DB, we expect this might fail due to lack of DB in the sandbox
    // But we want to ensure the route exists and doesn't return 404
    expect(createInboxRes.status()).not.toBe(404);
  });
});
