import { test, expect } from '@playwright/test';

test.describe('OnboardingWizard CUJ', () => {
  test.beforeEach(async ({ page, context }) => {
    // Clear local storage to ensure fresh state
    await page.addInitScript(() => {
      window.localStorage.clear();
    });
  });

  test('User can complete onboarding via Zero-Click Chat Agent', async ({ page }) => {
    await page.goto('/onboarding');

    // We start at step 0 which is the chat UI
    await expect(page.getByText("What do you want to build or manage today?")).toBeVisible();

    // Click the predefined chip
    await page.getByText('Cake Shop', { exact: true }).click();

    // Check if the chat input is there and wait for the mock reply
    // Send another message
    await page.getByPlaceholder('Type a message...').fill('Yes');
    await page.getByRole('button', { name: 'Send' }).click();
  });
});
