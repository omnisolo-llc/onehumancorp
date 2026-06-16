import { test, expect } from '../fixtures';

test.describe('Dashboard Milestone E2E', () => {
  test('verify milestone success card appears after logging in naturally', async ({ page }) => {
    await page.goto('/login');

    // We expect the user to see the dashboard normally
    // Using the real credentials and real data path. The database seed contains:
    // ms_e2e_10th_order for e2e-tenant. So the 10th order milestone should be visible.
    await page.getByPlaceholder('Email').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Sign in' }).click();

    await page.waitForURL('**/dashboard**');

    const milestoneCard = page.getByTestId('success-milestone-alert');
    // Using strict assertions since the seed is deterministic and is checked for real DB values.
    // If it was already dismissed in the shared runner we evaluate setting it back.
    await page.evaluate(() => {
        localStorage.setItem('dismissed_milestone_10th_order', 'false');
    });
    // Need to trigger a reload or checkMilestones after setting it.
    await page.reload();
    await page.waitForLoadState('networkidle');

    await expect(milestoneCard).toBeVisible({ timeout: 15000 });
    const textContent = await page.locator('.milestone-badge').first().textContent();
    expect(textContent).toContain('Milestone Unlocked');
  });
});
