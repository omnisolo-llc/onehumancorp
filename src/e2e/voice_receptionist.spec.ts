import { test, expect } from '@playwright/test';

test.describe('AI Voice Receptionist', () => {
  test('Enables voice receptionist, tests it, and checks inbox', async ({ page }) => {
    // Mock user login
    await page.route('**/api/v1/auth/login', async (route) => {
      await route.fulfill({ status: 200, json: { token: 'test-token', user: { id: 'test-user', organization_id: 'test-org' } } });
    });

    // Mock API requests
    await page.route('**/api/v1/dashboard/metrics', async (route) => {
      await route.fulfill({ status: 200, json: { todays_sales: 100, active_customers: 10, pending_orders: 2, messages_handled: 5 } });
    });

    await page.route('**/api/billing/cost-dashboard', async (route) => {
        await route.fulfill({ status: 200, json: []});
    });

    await page.route('**/api/v1/growth/team-invites/metrics*', async (route) => {
      await route.fulfill({ status: 200, json: { count: 0 } });
    });

    await page.route('**/api/agents/approvals', async (route) => {
      await route.fulfill({ status: 200, json: [] });
    });

    // Intercept the test voice request
    await page.route('**/api/voice/test', async (route) => {
      await route.fulfill({ status: 200, json: { success: true } });
    });

    // Intercept inbox messages
    await page.route('**/api/inbox/messages', async (route) => {
      await route.fulfill({ status: 200, json: [
        {
          id: "msg_voice_test_123",
          tenant_id: "test-org",
          source: "Voice",
          content: "AI Summary: Caller wants a plumbing quote. Sent booking link via SMS.\n\nTranscript: \nCustomer: Hi, I need a plumbing quote.\nAI: Hello! I can help with that. I've sent a booking link to your phone.",
          status: "handled",
          created_at: new Date().toISOString()
        }
      ]});
    });

    // Go to Dashboard
    await page.goto('/dashboard');

    // Toggle the Voice Receptionist
    const toggle = page.locator('text=Zero-Drop Calls').locator('xpath=../..').locator('button');
    await toggle.waitFor({ state: 'visible' });
    await toggle.click();

    // Click the test button
    const testBtn = page.get_by_text('Test my AI Receptionist');
    await testBtn.waitFor({ state: 'visible' });
    await testBtn.click();

    // Verify success message appears
    await expect(page.getByText('Call completed! Check your Inbox for details.')).toBeVisible();

    // Go to Inbox
    await page.goto('/inbox');

    // Check if the Voice message and its transcript appear
    await expect(page.getByText('AI Summary:')).toBeVisible();
    await expect(page.getByText('Caller wants a plumbing quote')).toBeVisible();
    await expect(page.getByText('Transcript:')).toBeVisible();
  });
});
