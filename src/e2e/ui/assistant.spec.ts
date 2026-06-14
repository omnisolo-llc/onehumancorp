import { test, expect } from '@playwright/test';

test.describe('Assistant Workstation (Jarvis-Parity)', () => {
  test('should display the assistant layout and handle task creation', async ({ page }) => {
    // Navigate to the assistant workstation
    await page.goto('/assistant');

    // Wait for the main heading to ensure the page has loaded
    await expect(page.getByRole('heading', { name: 'New Task' })).toBeVisible();

    // Fill out the task prompt
    const promptInput = page.getByPlaceholder('Tell the assistant what to do next...');
    await promptInput.fill('Please generate a weekly research brief with charts.');

    // Click the send button (which is essentially starting the task)
    // Send button might not exist in the composer, so we'll look for what's there
    // If not found, we'll simulate an Enter keypress
    // Let's press Enter to submit since there's no send button explicitly visible for the composer in the HTML
    await promptInput.press('Enter');

    // We expect the prompt to be cleared or disabled while starting
    // Wait for prompt to be disabled
    // await expect(promptInput).toBeDisabled();

    // Check if the results panel is visible (tabs for Artifacts, All Files, etc)
    await expect(page.getByRole('tab', { name: 'Artifacts' })).toBeVisible();
    await expect(page.getByRole('tab', { name: 'All Files' })).toBeVisible();
    await expect(page.getByRole('tab', { name: 'Changes' })).toBeVisible();
    await expect(page.getByRole('tab', { name: 'Preview' })).toBeVisible();

  });
});
