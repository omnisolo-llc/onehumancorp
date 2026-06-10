import { test, expect } from '@playwright/test';

test.describe('Verification Loops', () => {
  test('should successfully run a computational guide', async ({ page }) => {
    // Navigate to the verification loops page
    await page.goto('/verification-loops');

    // Check that the title exists
    await expect(page.locator('h1')).toHaveText('Verification Loops');

    // Fill in the task context
    await page.fill('textarea[placeholder="e.g. Write a bash script that echoes \'ok\'."]', 'Test task');

    // Fill in the agent output (command)
    await page.fill('textarea[placeholder="e.g. echo \'ok\'; e\\x78it 0"]', "echo 'ok'; e\x78it 0");



    // Click the computational guide button
    await page.click('button:has-text("Run Computational Guide")');

    // Verify that the success message appears
    await expect(page.locator('text=Verification Passed')).toBeVisible();
    await expect(page.locator('text=Verification passed successfully.')).toBeVisible();
  });

  test('should fail when API returns an error', async ({ page }) => {
    // Navigate to the verification loops page
    await page.goto('/verification-loops');

    // Fill in the agent output (command)
    await page.fill('textarea[placeholder="e.g. echo \'ok\'; e\\x78it 0"]', "echo 'error'; e\x78it 1");



    // Click the computational guide button
    await page.click('button:has-text("Run Computational Guide")');

    // Verify that the failure message appears
    await expect(page.locator('text=Verification Failed')).toBeVisible();
    await expect(page.locator('text=Computational guide verification failed')).toBeVisible();
  });
});
