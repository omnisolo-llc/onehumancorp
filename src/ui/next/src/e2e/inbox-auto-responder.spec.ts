import { test, expect } from '@playwright/test';

test.describe('Inbox Auto Responder API', () => {
  test('should return correct draft reply for intent', async ({ request }) => {
    const response = await request.post('/api/v1/inbox/auto_reply', {
      data: { message: "Are you open today?" }
    });

    expect(response.ok()).toBeTruthy();

    const data = await response.json();
    expect(data.success).toBe(true);
    expect(data.draft_reply).toContain('open until 6 PM today');
  });

  test('should escalate unknown intents', async ({ request }) => {
    const response = await request.post('/api/v1/inbox/auto_reply', {
      data: { message: "What is the meaning of life?" }
    });

    expect(response.ok()).toBeTruthy();

    const data = await response.json();
    expect(data.success).toBe(true);
    expect(data.escalate).toBe(true);
    expect(data.draft_reply).toContain('escalating');
  });
});
