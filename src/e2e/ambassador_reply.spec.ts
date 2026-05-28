import { test, expect } from './fixtures';
import { Client } from 'pg';

test.describe('Silent Ambassador Action Feed', () => {
  test('verify Silent Ambassador UI renders and handles incoming messages correctly', async ({ page, request }) => {
    // 1. Sign in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.getByRole('button', { name: /Login|Sign In/i }).filter({ visible: true }).first().click();

    await page.waitForURL('**/*');

    // Simulate incoming message to generate the real event
    // using the webhook route that handles external messages
    const webhookRes = await request.post('/api/agents/webhook', {
      data: {
        tenant_id: 'e2e-tenant',
        message: 'Can I order a gluten-free cake for tomorrow?',
        source: 'instagram'
      }
    });

    expect(webhookRes.status()).toBe(200);

    // Give the backend a second to process the webhook and generate the draft
    await page.waitForTimeout(4000);
    await page.reload();
    await page.waitForURL('**/*');

    await expect(page.getByText("Action Required")).toBeVisible({ timeout: 15000 });
    await expect(page.getByText("Customer Message")).toBeVisible({ timeout: 15000 });

    // Ensure our specific simulated message appeared in the Action Feed
    await expect(page.getByText("Can I order a gluten-free cake for tomorrow?")).toBeVisible({ timeout: 15000 });

    // Ensure the pre-seeded one is also visible or that there's an Approve & Send button
    const approveSendBtns = page.getByRole('button', { name: 'Approve & Send' });
    const count = await approveSendBtns.count();
    expect(count).toBeGreaterThan(0);

    // Approve the new message (it will be one of the approve buttons)
    await approveSendBtns.first().click();

    // Give it a moment to process the UI update
    await page.waitForTimeout(500);
  });
});
