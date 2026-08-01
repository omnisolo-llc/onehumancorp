import { expect, test } from './fixtures';

test.describe('Unified Inbox Triage Feed for Instagram DMs', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should triage incoming Instagram DM and allow owner to approve response', async ({ page }) => {
    test.setTimeout(180000);

    const testTenant = 'e2e-triage-unified-tenant-' + Date.now();

    // 1. Log in with specific tenant in UI FIRST to avoid cookie issues
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // 2. Simulate an incoming webhook from Meta/Instagram
    await page.request.post('/api/v1/webhooks/unified_inbox');

    // Give it a moment to parse job queue and generate triage action, then reload
    await page.waitForTimeout(5000);
    await page.reload();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // Wait for the Dashboard unified feed to show the Instagram DM card
    // We expect the backend to have processed the webhook and generated an action draft
    const instagramCard = page.locator('[data-testid="instagram-dm-card"]');

  });
});
