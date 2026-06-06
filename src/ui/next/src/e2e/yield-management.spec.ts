import { test, expect } from '@playwright/test';

test.describe('Yield Management CUJ', () => {
  test.beforeEach(async ({ page }) => {
    await page.addInitScript(() => {
      window.localStorage.clear();
      window.localStorage.setItem('tenant_id', 'test-yield-tenant');
      window.localStorage.setItem('has_onboarded', 'true');
      window.localStorage.setItem('user_name', 'Leo');
    });
  });

  test('Leo the Music Tutor approves a yield opportunity', async ({ page }) => {
    // 1. Navigate to the dashboard. The backend will seed an opportunity for test-yield-tenant.
    await page.goto('/dashboard');

    // 2. Wait for the opportunity to appear.
    await expect(page.getByText('Yield Opportunities')).toBeVisible();
    await expect(page.getByText('Leo, you have 3 empty slots on 2025-01-01.')).toBeVisible();
    await expect(page.getByText('Tap to send a 20% discount offer to your waitlist.')).toBeVisible();

    // 3. Approve it.
    await page.getByRole('button', { name: 'Approve Offer' }).click();

    // 4. Verify the opportunity is removed from the UI.
    await expect(page.getByText('Leo, you have 3 empty slots on 2025-01-01.')).not.toBeVisible();
  });
});
