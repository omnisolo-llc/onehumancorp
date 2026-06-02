import { test, expect } from '@playwright/test';

test.describe('Omni-Inbox API Webhook', () => {
  test('returns correct Ambassador reply for open hours', async ({ request }) => {
    const response = await request.post('/api/inbox/webhook', {
      data: {
        message: 'What time are you open until?',
        tenantId: 'test_tenant'
      }
    });
    expect(response.ok()).toBeTruthy();
    const body = await response.json();
    expect(body.agent).toBe('The Ambassador');
    expect(body.reply).toContain('we are open until 6 PM');
    expect(body.requiresHumanEscalation).toBe(false);
  });

  test('returns correct Ambassador reply for vegan allergy', async ({ request }) => {
    const response = await request.post('/api/inbox/webhook', {
      data: {
        message: 'Do you have vegan options?',
        tenantId: 'test_tenant'
      }
    });
    expect(response.ok()).toBeTruthy();
    const body = await response.json();
    expect(body.agent).toBe('The Ambassador');
    expect(body.reply).toContain('vegan birthday cake options');
    expect(body.requiresHumanEscalation).toBe(false);
  });
});
