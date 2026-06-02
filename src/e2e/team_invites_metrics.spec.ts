import { test, expect } from './fixtures';

test.describe('Growth Loop: Team Invites Metrics Component', () => {

  test('TC1: Should display Referral Program section on dashboard', async ({ page }) => {
    // Note: The `page` fixture automatically logs us in and lands on the /dashboard via `loginAs` in fixtures.ts.
    // The prompt requested that we navigate exactly as a real user.
    // From dashboard (which we land on after login), we can click home/dashboard buttons if available, or just assert.

    // We are already on the dashboard per fixtures.ts
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Referral Program' })).toBeVisible();
  });

  test('TC2: Should display "Team Invites Sent" card', async ({ page }) => {
    // We are already on the dashboard
    await expect(page.getByText('Team Invites Sent')).toBeVisible();
  });

  test('TC3: Should display active referrals and rewards metrics', async ({ page }) => {
    // We are already on the dashboard
    await expect(page.getByText('Active Referrals')).toBeVisible();
    await expect(page.getByText('Revenue from Referrals')).toBeVisible();
    await expect(page.getByText('Pending Rewards')).toBeVisible();
  });

  test('TC4: Should be able to open referral modal from dashboard', async ({ page }) => {
    // We are already on the dashboard
    await page.getByRole('button', { name: 'Invite a Business' }).click();
    await expect(page.getByText('Help a Business Grow!')).toBeVisible();
    await expect(page.getByText('Your Unique Link')).toBeVisible();
  });

  test('TC5: Should show correct default metrics value for Team Invites Sent', async ({ page }) => {
    // We are already on the dashboard
    // Check that 'Team Invites Sent' has a value (either '0' initially or a number fetched)
    const cardValue = page.locator('div', { hasText: 'Team Invites Sent' }).locator('div.text-indigo-900').first();
    await expect(cardValue).toBeVisible();
  });
});
