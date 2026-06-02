import { test, expect } from '@playwright/test';

test.describe('Voice Agent Dashboard CUJ', () => {
  test('SMB Owner can configure AI Voice Agent and view calls', async ({ page }) => {
    // Navigate to the new voice agent page
    await page.goto('/voice-agent');

    // Verify header and initial state
    await expect(page.locator('h1')).toHaveText('Voice Agent');
    await expect(page.getByText('Your Business Phone Number:')).toBeVisible();

    // Toggle the Voice Agent on
    const toggle = page.locator('input[type="checkbox"]').first();
    await toggle.check({ force: true });

    // Set a custom instruction
    const instructionsTextarea = page.locator('textarea');
    await instructionsTextarea.fill('Tell callers to park in the back near the blue door.');

    // Save configuration
    page.on('dialog', dialog => dialog.accept()); // Accept alert
    await page.getByRole('button', { name: 'Save Configuration' }).click();

    // Verify history section
    await expect(page.getByText('Call History & Transcripts')).toBeVisible();

    // Check that we can see the mock calls
    await expect(page.getByText('Maya G.')).toBeVisible();
    await expect(page.getByText('Carlos R.')).toBeVisible();

    // Expand the first call summary
    const firstCallDetails = page.locator('details').first();
    await firstCallDetails.click();
    await expect(page.getByText('Asked about vegan cake options.')).toBeVisible();

    // Refresh the page and ensure state persisted via local storage
    await page.reload();
    await expect(instructionsTextarea).toHaveValue('Tell callers to park in the back near the blue door.');
  });
});
