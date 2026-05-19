import { test, expect } from './fixtures';

test.describe('Autonomous Operations CUJ', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await expect(page).toHaveTitle(/OneHuman/);
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('shows the operational dashboard summary', async ({ page }) => {
    await expect(page.getByText('Business Snapshot')).toBeVisible();
    await expect(page.getByText('Orders to Ship')).toBeVisible();
    await expect(page.getByText('Team Members')).toBeVisible();
    await expect(page.getByText('Needs Your Approval')).toBeVisible();
  });

  test('surfaces the helper activity feed', async ({ page }) => {
    await expect(page.getByText('Agent Activity')).toBeVisible();
    await expect(page.locator('#agent-activity-feed')).not.toContainText('No recent activity.');
  });

  test('routes approvals-related work to the agents screen', async ({ page }) => {
    await page.getByRole('button', { name: 'Manage AI Assistants' }).click();

    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
    await expect(page.getByText('Marketing Pro')).toBeVisible();
    await expect(page.getByText('Status: Active').first()).toBeVisible();
  });

  test('keeps milestone acknowledgement interactive', async ({ page }) => {
    await page.getByRole('button', { name: 'Mark Order Ready' }).click();

    await expect(page.locator('#milestone-card')).toBeVisible();
    await expect(page.locator('#milestone-title')).toContainText('First Sale');
    await page.getByRole('button', { name: 'Dismiss' }).click();
    await expect(page.locator('#milestone-card')).toBeHidden();
  });

  test('keeps dashboard status language visible', async ({ page }) => {
    await expect(page.getByText('My Business:')).toBeVisible();
    await expect(page.getByText('Active').first()).toBeVisible();
    await expect(page.getByText('Your agents are working on your behalf.')).toBeVisible();
  });
});
