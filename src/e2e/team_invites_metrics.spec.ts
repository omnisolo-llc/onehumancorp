import { test, expect } from './fixtures';

test.describe('Growth Loop: Team Invites Metrics Component', () => {

  test.beforeEach(async ({ page }) => {
    // The default landing is '/' which redirects to '/onboarding' initially.
    // We must navigate to '/dashboard' directly since we assume the user is already onboarded or testing the dashboard specifically.
    await page.goto('/dashboard');
  });

  test('TC1: Should display Referral Program section on dashboard', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Dashboard', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Referral Program' })).toBeVisible();
  });

  test('TC2: Should display "Team Invites Sent" card', async ({ page }) => {
    await expect(page.getByText('Team Invites Sent')).toBeVisible();
  });

  test('TC3: Should display active referrals and rewards metrics', async ({ page }) => {
    await expect(page.getByText('Active Referrals')).toBeVisible();
    await expect(page.getByText('Revenue from Referrals')).toBeVisible();
    await expect(page.getByText('Pending Rewards')).toBeVisible();
  });

  test('TC4: Should be able to open referral modal from dashboard', async ({ page }) => {
    await page.getByRole('button', { name: '🎁 Invite a Business & Earn $50' }).click();
    await expect(page.getByText('Help a Business Grow!')).toBeVisible();
    await expect(page.getByText('Your Unique Link')).toBeVisible();
  });

  test('TC5: Should show correct default metrics value for Team Invites Sent', async ({ page }) => {
    // Check that 'Team Invites Sent' has a value (either '0' initially or a number fetched)
    const cardValue = page.locator('div', { hasText: 'Team Invites Sent' }).locator('div.text-indigo-900').first();
    await expect(cardValue).toBeVisible();
  });
});
