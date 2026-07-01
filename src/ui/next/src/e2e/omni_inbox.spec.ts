import { test, expect } from '@playwright/test';

test.describe('Omnichannel Inbox UI', () => {
  test('Owner sees sender id and known customer in inbox', async ({ page, request }) => {
    // 1. Send webhook
    const res = await request.post('/api/v1/webhooks/unified_inbox', {
      data: {
        tenant_id: "e2e-tenant",
        source: "instagram",
        identifier: "maya_bakes",
        message: "Do you have vegan options?"
      }
    });
    expect(res.ok()).toBeTruthy();

    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    await page.goto('/inbox');

    const msg = page.locator('.app-list-item', { hasText: 'Do you have vegan options' }).first();
    await expect(msg).toBeVisible({ timeout: 10000 });
    await msg.click();

    await expect(page.getByText('maya_bakes')).toBeVisible();
    await expect(page.getByText('Known Customer')).toBeVisible();
  });

  test('Owner sees AI-drafted contextual reply in unified inbox', async ({ page, request }) => {
    // 1. Trigger the webhook
    const res = await request.post('/api/v1/webhooks/unified_inbox', {
      data: {
        tenant_id: "e2e-tenant",
        source: "instagram",
        identifier: "maya_bakes",
        message: "Hi, I am interested in ordering a cake."
      }
    });
    expect(res.ok()).toBeTruthy();

    // 2. Login
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // 3. Go to Inbox
    await page.goto('/inbox');

    // 4. Click the newly arrived message
    const msg = page.locator('.app-list-item', { hasText: 'Hi, I am interested in ordering a cake.' }).first();
    await expect(msg).toBeVisible({ timeout: 10000 });
    await msg.click();

    // 5. Verify the AI drafted reply appears (the backend uses the local LLM mock which contains "Hi there! Thanks for your message" or similar fallback, we just check for draft presence)
    await expect(page.getByText('Draft Reply')).toBeVisible();

    // 6. Verify "Approve & Send Draft" button is visible
    await expect(page.getByRole('button', { name: '✨ Approve & Send Draft' })).toBeVisible();
  });
});
