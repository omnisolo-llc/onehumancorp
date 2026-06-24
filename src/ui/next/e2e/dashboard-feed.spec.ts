import { test, expect } from '@playwright/test';

test.describe('Dashboard Actionable Feed', () => {
  test('should display database-backed operations console', async ({ page }) => {
    await page.goto('/dashboard');

    await expect(page.getByRole('heading', { name: 'Business Analytics' })).toBeVisible();
    await expect(page.locator('text="Operations Map"').first()).toBeVisible();
    await expect(page.locator('text="Action Required"').first()).toBeVisible();
    await expect(page.locator('text="Recent Orders"')).toBeVisible();
    await expect(page.locator('text="Inbox Activity"')).toBeVisible();
  });

  test('Assistant-first CUJ: Send Deposit Link from Triage', async ({ page }) => {
    // 1. User navigates to dashboard
    await page.goto('/dashboard');

    // 2. Action Center should be prominent
    await expect(page.getByRole('heading', { name: 'Action Center' })).toBeVisible();

    // 3. Since this is an E2E test, we assume the initial mocked data has an instagram DM card
    // or we intercept the API to provide one for the test. Let's try intercepting first to ensure it's there.
    await page.route('/api/agent-feed*', async route => {
      const json = {
        items: [
          {
            id: 'mock-ig-123',
            tenant_id: 'e2e-tenant',
            event_source: 'instagram',
            lifecycle_state: 'PENDING_APPROVAL',
            proposed_action: {
              feature_type: 'instagram_dm',
              customer_message: 'Hi Maya, I need a cake for Friday.',
              draft_reply: 'Sure! Please pay the deposit.',
            },
            created_at: new Date().toISOString()
          }
        ]
      };
      await route.fulfill({ json });
    });

    // Reload to apply mock
    await page.goto('/dashboard');

    // 4. Wait for the feed item to render
    const card = page.getByTestId('instagram-dm-card');
    await expect(card).toBeVisible();

    // 5. Look for the Send Deposit Link button
    const depositBtn = card.getByRole('button', { name: 'Send Deposit Link' });
    await expect(depositBtn).toBeVisible();

    // 6. Click it and ensure it triggers optimistic update (card disappears)
    await depositBtn.click();
    await expect(card).not.toBeVisible();
  });
});
