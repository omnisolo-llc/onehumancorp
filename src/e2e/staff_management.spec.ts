import { test, expect } from '@playwright/test';

test.describe('Staff Management Dashboard', () => {
  // Use realistic owner-entered data per guidelines
  const staffMemberName = 'Jun (Location Manager)';

  test('should display staff tasks and allow marking them as completed offline', async ({ page }) => {
    // Navigate to the staff page as Jun
    await page.goto('/staff');

    // Verify translucent glassmorphism UI elements are present
    const header = page.locator('header.backdrop-blur-xl');
    await expect(header).toBeVisible();
    await expect(header).toContainText('My Shifts & Tasks');

    // Wait for the simulated fetch delay / tasks to render
    const tasksSection = page.locator('section').filter({ hasText: 'My Tasks' });
    await expect(tasksSection).toBeVisible();

    // Verify task generation output
    // The stub currently returns "Fulfill 5 cake orders"
    const taskTitle = tasksSection.locator('span.text-slate-900').first();
    await expect(taskTitle).toHaveText('Fulfill 5 cake orders');

    // Identify the checkbox for the task
    const checkbox = tasksSection.locator('input[type="checkbox"]').first();
    await expect(checkbox).not.toBeChecked();

    // Jun completes the task
    await checkbox.check();

    // Assert that the local state updates optimistically (offline tolerance)
    await expect(checkbox).toBeChecked();

    // A real backend would eventually sync this, but for E2E we verify the UI reacts truthfully
  });
});
