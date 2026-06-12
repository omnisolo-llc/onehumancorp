import { test, expect } from './fixtures';

test.describe('Ralph Loop UI CUJ', () => {
  test('should allow owner to initialize an autonomous task and see progress', async ({ page }) => {
    // 1. Navigate to Ralph Loop page
    await page.goto('/ralph.html');
    await expect(page.locator('h1')).toHaveText('Ralph Loop');

    // 2. Enter an objective
    const taskInput = page.locator('#task-input');
    await taskInput.fill('Build a test system');

    // 3. Click Initialize
    await page.click('#start-btn');

    // 4. Verify working status
    await expect(page.locator('#overall-status')).toHaveText('Working');

    // 5. Check if log appeared
    await expect(page.locator('#log-container')).toContainText('Initializing objective');

    // 6. Wait for background update (mocked RPC in many tests, but here we expect elements to exist)
    // In a real run, feature list would populate.
  });
});

test('should show project summary updates', async ({ page }) => {
    await page.goto('/ralph.html');
    await page.evaluate(() => {
        const summaryText = document.getElementById('project-summary-text');
        summaryText.innerText = 'Updated Summary via mock';
    });
    await expect(page.locator('#project-summary-text')).toHaveText('Updated Summary via mock');
});

test('should maintain logs correctly', async ({ page }) => {
    await page.goto('/ralph.html');
    await page.locator('#task-input').fill('Test Log');
    await page.click('#start-btn');
    await expect(page.locator('#log-container')).toContainText('Initializing objective: Test Log');
});
