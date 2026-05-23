import { test, expect } from './fixtures';

test.describe('Agent Harness Isolation & Telemetry E2E', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await expect(page).toHaveTitle(/OneHuman/);
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('verifies agent operations surface correctly from the sandbox', async ({ page }) => {
    // Assert the dashboard snapshot confirms the presence of operations
    await expect(page.getByText('Business Snapshot')).toBeVisible();

    // Assert agent activity is properly populated on the dashboard,
    // which signifies the underlying sandbox harness processes are successfully
    // emitting telemetry and completing operations.
    await expect(page.getByText('Agent Activity')).toBeVisible();
    await expect(page.locator('#agent-activity-feed')).not.toContainText('No recent activity.');

    // Check specific agent presence that validates the harness connection
    await page.getByRole('button', { name: 'Manage AI Assistants' }).click();
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
    await expect(page.getByText('Status: Active').first()).toBeVisible();
  });
});
