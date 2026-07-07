import { test, expect } from '@playwright/test';

test.describe('Universal Autonomous Staff Management & Local Coordination Mesh', () => {
  test('CUJ: Staff member views, completes a task and escalates an issue', async ({ page }) => {
    await page.route('**/api/staff/tasks', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify([
          { id: 'task-1', description: 'Prepare 15 Falafels', status: 'PENDING', priority: 'HIGH' },
          { id: 'task-2', description: 'Check bathrooms', status: 'PENDING', priority: 'NORMAL' },
        ])
      });
    });

    await page.route('**/api/staff/shifts', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify([
          { id: '1', role: 'Baker', startTime: new Date().toISOString(), endTime: new Date(Date.now() + 8*3600000).toISOString(), status: 'Scheduled' }
        ])
      });
    });

    await page.route('**/api/staff/tasks/task-1', async route => {
      await route.fulfill({ status: 200, body: JSON.stringify({ success: true }) });
    });

    await page.goto('/staff');

    await expect(page.locator('h1', { hasText: 'My Shifts & Tasks' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Active Shift' })).toBeVisible();
    await expect(page.locator('text=Baker')).toBeVisible();

    await expect(page.locator('h2', { hasText: 'Action Items' })).toBeVisible();
    await expect(page.locator('text=Prepare 15 Falafels')).toBeVisible();
    await expect(page.locator('text=Check bathrooms')).toBeVisible();

    await expect(page.locator('text=Priority Action').first()).toBeVisible();

    const taskCheckbox = page.locator('input[type="checkbox"]').first();
    await taskCheckbox.click();
    await expect(taskCheckbox).toBeChecked();

    await expect(page.getByRole('button', { name: 'Escalate Issue' })).toBeVisible();
  });
});
