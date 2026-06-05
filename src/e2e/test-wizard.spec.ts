import { test, expect } from '@playwright/test';

test('Onboarding Wizard Basic Flow', async ({ page }) => {
  await page.goto('http://localhost:3000/onboarding');

  // Wait for the step 1 welcome screen
  await expect(page.getByText('Welcome')).toBeVisible();
});
