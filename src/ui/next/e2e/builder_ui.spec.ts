import { test, expect } from '@playwright/test';

test('builder flow completes successfully', async ({ page }) => {
  // Use the baseURL from playwright config (or relative to it if Next is served there)
  await page.goto('http://localhost:3000/builder');

  // 1. Onboarding Screen - Step 1: Basics
  await expect(page.getByText(/Let's build your store/i)).toBeVisible({ timeout: 15000 });

  const nameInput = page.getByPlaceholder(/e.g. Acme Corp/i);
  await nameInput.fill('My Awesome Store');

  const categoryInput = page.getByPlaceholder(/e.g. Retail, Consulting, Tech/i);
  await categoryInput.fill('Retail');

  await page.getByRole('button', { name: /Next: Choose Vibe/i }).click();

  // Step 2: Vibe
  await expect(page.getByText(/Select Your Vibe/i)).toBeVisible();
  await page.getByRole('button', { name: 'Friendly' }).click();
  await page.getByRole('button', { name: /Next: Details/i }).click();

  // Step 3: Final Details
  await expect(page.getByText(/Final Details/i)).toBeVisible();
  const textarea = page.getByPlaceholder(/e.g. I run a mobile dog grooming service/i);
  await expect(textarea).toBeVisible();

  // The bio should be pre-filled, but we can append or replace it
  await textarea.fill('I run a friendly retail store selling amazing products');

  // Click Generate
  const buildButton = page.getByRole('button', { name: /Build Store/i });
  await buildButton.click();

  // 2. Generating Screen
  await expect(page.getByText(/The Promoter is picking colors/i)).toBeVisible();

  // 3. Draft Preview Screen
  await expect(page.getByText(/Preview Mode/i)).toBeVisible({ timeout: 5000 });
  await expect(page.getByText(/1-Tap Launch/i)).toBeVisible();

  // 4. Click Launch
  await page.getByRole('button', { name: /1-Tap Launch/i }).click();

  // 5. Launch Screen
  await expect(page.getByText(/You're Live/i)).toBeVisible({ timeout: 5000 });
});
