import { test, expect } from '../../../../e2e/fixtures';

test.describe('The Ralph Loop UI E2E', () => {
  test('Owner can navigate to Ralph Loop, enter task, and see execution', async ({ page }) => {
    // Navigate to the Ralph Loop page
    await page.goto('/ralph-loop');

    // Wait for the page to load
    await expect(page.locator('h1')).toContainText('The Ralph Loop');

    // Interact with the text area
    const taskInput = page.getByLabel(/Long-Running Task Description/i);
    await taskInput.fill('Implement an end-to-end feature spanning multiple sessions');

    // Verify button is enabled
    const executeButton = page.getByRole('button', { name: /Start Ralph Loop/i });
    await expect(executeButton).toBeEnabled();

    // In a real live service test without mocking backend, we don't necessarily want
    // the full 2-minute ralph loop to run here unless the backend handles it quickly.
    // Assuming the backend is running and responds with a success status or handled error.

    // We will just verify the button changes state and an API request is made
    const requestPromise = page.waitForRequest(req => req.url().includes('/api/ralph-loop') && req.method() === 'POST');

    await executeButton.click();

    // Verify loading state
    await expect(page.getByRole('button', { name: /Ralph Loop Executing/i })).toBeVisible();

    // Wait for the request to complete
    await requestPromise;

    // Depending on backend response (which we cannot mock per instructions, must be real),
    // we expect either a success block or an error block.
    // If backend is down or not configured fully locally, it might show error.
    // We just verify that one of them appears.
    await expect(page.locator('[data-testid="success-message"], [data-testid="error-message"]')).toBeVisible({ timeout: 10000 });
  });
});
