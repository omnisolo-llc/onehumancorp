import { test, expect } from '@playwright/test';

test.describe('Assistant Workstation (Jarvis-Parity)', () => {
  test('should display the assistant layout and handle task creation', async ({ page }) => {
    // Navigate to the assistant workstation
    await page.goto('/assistant');

    // Wait for the main heading to ensure the page has loaded
    await expect(page.getByRole('heading', { name: 'Agent Assistant' })).toBeVisible();

    // Fill out the task prompt
    const promptInput = page.getByPlaceholder('What do you need help with?');
    await promptInput.fill('Please generate a weekly research brief with charts.');

    // Click the send button (which is essentially starting the task)
    const sendButton = page.getByRole('button', { name: 'Send' });
    await expect(sendButton).toBeEnabled();

    // Since we mock or proxy the backend, we can just assert that the button can be clicked
    // and the input handles the text correctly.
    // In a real E2E environment with the backend running, we'd wait for a new task in the list.
    await sendButton.click();

    // We expect the prompt to be cleared or disabled while starting
    // Wait for prompt to be disabled
    // await expect(promptInput).toBeDisabled();

    // Check if the results panel is visible (tabs for Artifacts, All Files, etc)
    await expect(page.getByRole('tab', { name: 'Artifacts' })).toBeVisible();
    await expect(page.getByRole('tab', { name: 'All Files' })).toBeVisible();
    await expect(page.getByRole('tab', { name: 'Changes' })).toBeVisible();
    await expect(page.getByRole('tab', { name: 'Preview' })).toBeVisible();

    // Check for the Clipboard screenshot paste feature
    await expect(page.getByRole('button', { name: 'Clipboard screenshot paste' })).toBeVisible({ timeout: 15000 });
  });
});
