import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Staff Operations', () => {
  adminPage('Manager Jun can view staff shift and tasks, escalate, and see summary', async ({ page }) => {
    // We should not mock network requests, but instead use the actual seeded data or create records if needed.
    // The problem from earlier was playwright failure to load adminPage.
    // We'll test against the real backend without mocks.

    // Load the HTML staff view page which contains our JS making real requests
    await page.goto('/api/ui/staff.html');

    // Make sure we are on the page
    await expect(page.locator('h1', { hasText: 'My Shift' })).toBeVisible();

    // Verify Staff Tasks list loads
    const tasksContainer = page.locator('#task-list-container');
    await expect(tasksContainer).toBeVisible();

    // Trigger the Escalate Issue (low supplies)
    const escalateBtn = page.locator('#escalate-supplies-btn');
    await expect(escalateBtn).toBeVisible();
    await escalateBtn.click();

    // Switch to Manager View
    const managerTab = page.locator('.tab', { hasText: 'Manager View' });
    await managerTab.click();

    // Verify Shift Performance and Escalations sections
    await expect(page.locator('h2', { hasText: 'Shift Performance' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Escalations' })).toBeVisible();

    // Wait for escalations to populate
    const escalationsContainer = page.locator('#escalations-container');
    await expect(escalationsContainer).toBeVisible();

  });
});
