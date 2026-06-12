import { test, expect } from '@playwright/test';

test.describe('AgenticSeek Local Agent UI', () => {
  test('should render and submit a local task successfully', async ({ page }) => {
    // Intercept the API call to mock the local agent response
    await page.route('**/api/agents/agentic-seek', async (route) => {
      const request = route.request();
      if (request.method() === 'POST') {
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({ result: 'Successfully processed purely on local compute. AgenticSeek rules.' }),
        });
      } else {
        await route.continue();
      }
    });

    await page.goto('/agentic-seek');

    // Wait for the page to load
    await expect(page.locator('h1', { hasText: 'AgenticSeek Local Agent' })).toBeVisible();

    // Fill in the task
    const textarea = page.locator('textarea[placeholder="e.g. Analyze the local log files and summarize errors..."]');
    await textarea.fill('E2E Local Test Task');

    // Submit the task
    const button = page.locator('button', { hasText: 'Execute Local Task' });
    await expect(button).toBeEnabled();
    await button.click();

    // Verify loading state
    await expect(page.locator('button', { hasText: 'Running Locally...' })).toBeVisible();

    // Verify the result
    await expect(page.locator('h2', { hasText: 'Local Execution Result' })).toBeVisible();
    await expect(page.locator('pre', { hasText: 'Successfully processed purely on local compute. AgenticSeek rules.' })).toBeVisible();
  });

  test('should display error message on backend failure', async ({ page }) => {
    await page.route('**/api/agents/agentic-seek', async (route) => {
      await route.fulfill({
        status: 500,
        contentType: 'application/json',
        body: JSON.stringify({ error: 'Local backend failure' }),
      });
    });

    await page.goto('/agentic-seek');

    const textarea = page.locator('textarea[placeholder*="Analyze the local log files"]');
    await textarea.fill('Fail this task');

    const button = page.locator('button', { hasText: 'Execute Local Task' });
    await button.click();

    await expect(page.locator('h3', { hasText: 'Execution Error:' })).toBeVisible();
    await expect(page.locator('p', { hasText: 'Local backend failure' })).toBeVisible();
  });
});
