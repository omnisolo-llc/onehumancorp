import { test, expect } from '@playwright/test';

test.describe('Invoice AI Flow', () => {
  test('creates an invoice from natural language and routes to finance', async ({ request }) => {
    // We verify the AI routing directly maps an invoice natural language prompt to the Finance agent
    const res = await request.post('http://localhost:3000/api/agents/chat', {
        data: { message: "Send an invoice for $50 to John for plumbing repair" }
    });

    expect(res.ok()).toBeTruthy();
    const data = await res.json();

    expect(data.department_assigned).toBe('finance');
    expect(data.agent).toBe('The Accountant');
    expect(data.description).toContain('I have drafted an invoice based on your request. I will generate a Stripe payment link and send it via your preferred omnichannel route');
  });
});
