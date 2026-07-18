import { test, expect } from '@playwright/test';

test.describe('Gather-Act-Verify UI', () => {
  test('should display the form, allow submission, and handle errors correctly', async ({ page }) => {
    // Navigate to the page
    await page.goto('/gather-act-verify');

    // Verify header exists
    await expect(page.getByRole('heading', { name: 'Gather-Act-Verify Agent' })).toBeVisible();

    // Verify form elements exist
    const taskInput = page.getByLabel('Task Description');
    await expect(taskInput).toBeVisible();

    const submitBtn = page.getByRole('button', { name: 'Run Agent' });
    await expect(submitBtn).toBeVisible();
    await expect(submitBtn).toBeDisabled();

    // Fill the form
    await taskInput.fill('Analyze this codebase');
    await expect(submitBtn).toBeEnabled();

    // We are expecting an error since we don't have a backend mock, just verifying the UI handles it
    // Wait for the request and respond with a 500 error
    await page.route('/api/gather_act_verify', async (route) => {
      await route.fulfill({
        status: 500,
        contentType: 'application/json',
        body: JSON.stringify({ error: 'Failed to communicate with Gather Act Verify backend' })
      });
    });

    await submitBtn.click();

    // Should show loading state
    await expect(page.getByRole('button', { name: 'Processing...' })).toBeVisible();

    // Should show error message eventually
    await expect(page.getByText('Failed to process task')).toBeVisible();
  });
});
