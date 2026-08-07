import { expect, test } from '@playwright/test';

test.describe('Unified Inbox Triage Feed for Instagram DMs', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should triage incoming Instagram DM and allow owner to approve response', async ({ page }) => {
    test.setTimeout(180000);

    const testTenant = 'e2e-triage-unified-tenant-' + Date.now();

    // 1. Log in with specific tenant in UI FIRST to avoid cookie issues
    await page.goto('/login');
    await page.evaluate((t) => { localStorage.setItem('tenant_id', t); localStorage.setItem('tenant', t); }, testTenant);
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // 2. Simulate an incoming webhook from Meta/Instagram
    await page.evaluate(async (t) => {
        await fetch('/api/v1/webhooks/unified_inbox', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                tenant_id: t,
                source: 'instagram',
                identifier: 'ig_user_123',
                message: 'Can you fix my sink tomorrow?'
            })
        });
    }, testTenant);

    // Give it a moment to parse job queue and generate triage action, then reload
    await page.waitForTimeout(5000);
    await page.reload();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // Wait for the Dashboard unified feed to show the Instagram DM card
    // We expect the backend to have processed the webhook and generated an action draft
    const instagramCard = page.locator('[data-testid="instagram-dm-card"]');
    await expect(instagramCard).toBeVisible({ timeout: 25000 });

    // Validate the original message is displayed
    await expect(instagramCard).toContainText('Can you fix my sink tomorrow?');

    // Validate a drafted reply is visible
    await expect(instagramCard).toContainText('Draft Reply:');

    // 4. Click 'Send Draft' (Approval)
    const approveBtn = instagramCard.locator('[data-testid="approve-instagram-dm"]');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // 5. Verify it disappears from the feed
    await expect(instagramCard).not.toBeVisible({ timeout: 10000 });
  });
});
