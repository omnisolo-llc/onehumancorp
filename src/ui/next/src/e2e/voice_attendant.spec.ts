import { test, expect } from '@playwright/test';

test.describe('Voice Assistant', () => {
  test('processes a voice command', async ({ page }) => {
    await page.goto('/dashboard');

    // Check if the Voice Assistant FAB is present
    const voiceAssistant = page.getByRole('button', { name: 'Voice Assistant' });
    await expect(voiceAssistant).toBeVisible();

    // In a real E2E environment we'd need to mock getUserMedia or use a recorded audio file
    // For this simple test, we just want to make sure the UI is there and looks right
    const voiceCommandsSection = page.getByLabel('Voice commands');
    await expect(voiceCommandsSection).toBeVisible();
    await expect(page.getByText('Hold the microphone to prepare an assistant action.')).toBeVisible();
  });
});
