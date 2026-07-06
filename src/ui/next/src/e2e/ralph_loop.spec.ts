import { test, expect } from '@playwright/test';

test.describe('Ralph Loop UI CUJ', () => {
  test.beforeEach(async ({ page }) => {
    // Go to the Ralph Loop page
    await page.goto('/ralph-loop');
  });

  test('should render the Ralph Loop page correctly', async ({ page }) => {
    await expect(page.locator('h1', { hasText: 'The Ralph Loop (Long-Running Agent)' })).toBeVisible();
    await expect(page.locator('label[for="task"]')).toBeVisible();
    await expect(page.locator('label[for="progress_file"]')).toBeVisible();
    await expect(page.locator('button', { hasText: 'Start Ralph Loop' })).toBeVisible();
  });

  test('button should be disabled when task is empty', async ({ page }) => {
    const startButton = page.locator('button', { hasText: 'Start Ralph Loop' });
    await expect(startButton).toBeDisabled();

    // Fill the task
    await page.fill('#task', 'Build a web server');
    await expect(startButton).toBeEnabled();

    // Clear the task
    await page.fill('#task', '   ');
    await expect(startButton).toBeDisabled();
  });

  test('should display loading state during execution', async ({ page }) => {
    // We mock the API response to delay so we can see the loading state
    await page.route('/api/ralph-loop', async route => {
      // Delay for 500ms
      await new Promise(resolve => setTimeout(resolve, 500));
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ result: { status: 'success' } })
      });
    });

    await page.fill('#task', 'Test long running task');
    const startButton = page.locator('button', { hasText: 'Start Ralph Loop' });
    await startButton.click();

    // Check loading state
    await expect(page.locator('button', { hasText: 'Ralph Loop Executing (Check terminal/git)...' })).toBeVisible();
    await expect(page.locator('button', { hasText: 'Ralph Loop Executing (Check terminal/git)...' })).toBeDisabled();

    // Wait for success message
    await expect(page.getByTestId('success-message')).toBeVisible();
  });

  test('should display error message on API failure', async ({ page }) => {
    await page.route('/api/ralph-loop', async route => {
      await route.fulfill({
        status: 500,
        contentType: 'application/json',
        body: JSON.stringify({ error: 'Simulated backend failure' })
      });
    });

    await page.fill('#task', 'Test error handling');
    await page.click('button:has-text("Start Ralph Loop")');

    // Check error state
    await expect(page.getByTestId('error-message')).toBeVisible();
    await expect(page.getByTestId('error-message')).toContainText('Execution Error');
    await expect(page.getByTestId('error-message')).toContainText('Simulated backend failure');
  });

  test('should display result successfully on API success', async ({ page }) => {
    const mockResult = {
      task_description: "Build a web server",
      features: [
        { name: "Step 1", status: "completed" },
        { name: "Step 2", status: "pending" }
      ],
      current_feature_index: 1,
      notes: ["Completed Step 1"],
      is_complete: false
    };

    await page.route('/api/ralph-loop', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ result: mockResult })
      });
    });

    await page.fill('#task', 'Build a web server');
    await page.fill('#progress_file', '.custom_progress.json');
    await page.click('button:has-text("Start Ralph Loop")');

    // Check success state
    await expect(page.getByTestId('success-message')).toBeVisible();
    const resultText = await page.locator('pre').textContent();
    expect(resultText).toContain('"task_description": "Build a web server"');
    expect(resultText).toContain('"status": "completed"');

    // Ensure error is not visible
    await expect(page.getByTestId('error-message')).toBeHidden();
  });
});
