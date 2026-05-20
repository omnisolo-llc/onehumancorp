import { test, expect } from '@playwright/test';

test('builder flow completes successfully', async ({ page }) => {
  // Use the baseURL from playwright config (or relative to it if Next is served there)
  await page.goto('/builder');

  // 1. Onboarding Screen
  const textarea = page.getByPlaceholder(/e.g. I run a mobile dog grooming service/i);
  await expect(textarea).toBeVisible();

  // Fill the bio
  await textarea.fill('I run a mobile dog grooming service');

  // Click Generate
  const buildButton = page.getByRole('button', { name: /Build My Storefront/i });
  await buildButton.click();

  // 2. Generating Screen
  await expect(page.getByText(/The Promoter is picking colors/i)).toBeVisible();

  // 3. Draft Preview Screen
  await expect(page.getByText(/Welcome/i)).toBeVisible({ timeout: 10000 });
  await expect(page.getByText(/1-Tap Launch/i)).toBeVisible();

  // 4. Click Launch
  await page.getByRole('button', { name: /1-Tap Launch/i }).click();

  // 5. Launch Screen
  await expect(page.getByText(/You're Live/i)).toBeVisible({ timeout: 5000 });
});
