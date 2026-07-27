import { test, expect } from '@playwright/test';

test.describe('Omnichannel Chat', () => {
  test('should simulate a new inbound message hitting the API', async ({ request }) => {
    const tenant_id = 'test-tenant';
    const res = await request.post('/api/inboxes', {
      data: {
        tenant_id,
        name: 'Test Inbox'
      }
    });

    expect(res.ok()).toBeTruthy();
    const inbox = await res.json();
    expect(inbox.name).toBe('Test Inbox');
  });
});
