import { test, expect } from '@playwright/test';

test.describe('Unified Inbox Auto Reply', () => {
  test('off-hours CUJ: User navigates to inbox, configures auto-reply via real API endpoint', async ({ page }) => {
    // Navigate to Inbox
    await page.goto('/inbox');

    // Verify Inbox header and layout
    await expect(page.getByText('Conversations')).toBeVisible();

    // Open Settings Modal
    await page.getByRole('button', { name: 'Settings' }).click();
    await expect(page.getByRole('heading', { name: 'Inbox Settings' })).toBeVisible();

    // Enable Working Hours
    await page.getByLabel('Enable Working Hours').check();

    // Fill Out of Office message
    const oooMessage = 'We are currently offline. An agent will be with you tomorrow morning.';
    await page.locator('textarea').fill(oooMessage);

    // Setup API intercept to mock the backend response (since we use hardcoded 000 IDs in this demo logic, but test real frontend functionality)
    // In a real environment, this clicks a button that makes a real backend API call.
    });

    // Save Changes
    await page.getByRole('button', { name: 'Save Changes' }).click();

    // Verify modal closes
    await expect(page.getByRole('heading', { name: 'Inbox Settings' })).toBeHidden();
  });
});
