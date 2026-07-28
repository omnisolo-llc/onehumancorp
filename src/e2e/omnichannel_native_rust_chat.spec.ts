import { expect, test } from '@playwright/test';

test.describe('Omnichannel Native Rust Chat', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should display incoming native chat messages and allow the owner to see them', async ({ page }) => {
    test.setTimeout(180000);

    const testTenant = 'e2e-omnichannel-native-' + Date.now();

    // 1. Log in with specific tenant
    await page.goto('/login');
    await page.evaluate((t) => { localStorage.setItem('tenant_id', t); localStorage.setItem('tenant', t); }, testTenant);
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // 2. We mock an incoming webhook since this feature is newly implemented and we need to simulate the external trigger
    await page.evaluate(async (t) => {
        // Native chat webhook test (Mock)
        await fetch('/api/v1/webhooks/unified_inbox', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                tenant_id: t,
                source: 'whatsapp',
                identifier: 'wa_user_123',
                message: 'Hello from native rust chat engine!'
            })
        });
    }, testTenant);

    // 3. Navigate to Unified Inbox
    await page.goto('/inbox');

    // Wait for the Dashboard unified feed to show the WhatsApp message card
    const whatsappCard = page.locator('text=Hello from native rust chat engine!');
    await expect(whatsappCard).toBeVisible({ timeout: 25000 });
  });
});
