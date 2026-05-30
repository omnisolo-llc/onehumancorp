import { test, expect } from './fixtures';

test('verify onboarding step 1 UI structure', async ({ page }) => {
  await page.goto('/onboarding');
  await expect(page.getByText("What's the name of your business?")).toBeVisible();
  const nextBtn = page.getByRole('button', { name: /Next/i });
  await expect(nextBtn).toBeDisabled();
});

test('verify onboarding allows name input and navigation', async ({ page }) => {
  await page.goto('/onboarding');
  const nameInput = page.getByPlaceholder(/Maya's Custom Cakes/i);
  await nameInput.fill('Maya Bakery');
  const nextBtn = page.getByRole('button', { name: /Next/i });
  await expect(nextBtn).toBeEnabled();
  await nextBtn.click();
  await expect(page.getByPlaceholder(/I bake custom vegan cakes/i)).toBeVisible();
});

test('verify onboarding business description input', async ({ page }) => {
  await page.goto('/onboarding');
  await page.getByPlaceholder(/Maya's Custom Cakes/i).fill('Maya Bakery');
  await page.getByRole('button', { name: /Next/i }).click();
  const sellInput = page.getByPlaceholder(/I bake custom vegan cakes/i);
  await sellInput.fill('Cakes');
  const nextBtn = page.getByRole('button', { name: /Next/i });
  await expect(nextBtn).toBeEnabled();
  await nextBtn.click();
  await expect(page.getByPlaceholder(/Portland, OR/i)).toBeVisible();
});

test('verify onboarding location input and intake button', async ({ page }) => {
  await page.goto('/onboarding');
  await page.getByPlaceholder(/Maya's Custom Cakes/i).fill('Maya Bakery');
  await page.getByRole('button', { name: /Next/i }).click();
  await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Cakes');
  await page.getByRole('button', { name: /Next/i }).click();

  const locInput = page.getByPlaceholder(/Portland, OR/i);
  await locInput.fill('NY');

  const generateBtn = page.getByRole('button', { name: /Generate My Business/i });
  await expect(generateBtn).toBeEnabled();
});

test('verify onboarding goes back to previous steps', async ({ page }) => {
  await page.goto('/onboarding');
  await page.getByPlaceholder(/Maya's Custom Cakes/i).fill('Maya Bakery');
  await page.getByRole('button', { name: /Next/i }).click();
  await expect(page.getByPlaceholder(/I bake custom vegan cakes/i)).toBeVisible();

  // Go back to step 1
  await page.getByRole('button', { name: /Back/i }).click();
  await expect(page.getByPlaceholder(/Maya's Custom Cakes/i)).toBeVisible();
});
