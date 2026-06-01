import { expect, test } from './fixtures';

test('builder wizard flow to AI Architect', async ({ page }) => {
  await page.goto('/builder');

  // Verify Onboarding Screen
  await expect(page.getByText('What are you building today?')).toBeVisible();

  // Pick a goal
  await page.getByText('Selling Products').click();

  // Verify Step 1
  await expect(page.getByText("Let's build your store")).toBeVisible();

  // Fill Step 1
  await page.getByPlaceholder('e.g. Acme Corp').fill('Maya Bakery');
  await page.getByPlaceholder('e.g. Retail, Consulting, Tech').fill('Food and Beverage');
  await page.getByText('Next: Choose Vibe').click();

  // Verify Step 2 and select vibe
  await expect(page.getByText("Select Your Vibe")).toBeVisible();
  await page.getByText('Friendly').click();
  await page.getByText('Next: Details').click();

  // Verify Step 3 and write bio
  await expect(page.getByText("Final Details")).toBeVisible();
  await page.getByPlaceholder(/e\.g\. I run a mobile dog grooming/i).fill('I sell delicious cupcakes.');

  // Build Store
  await page.getByText('Build Store').click();

  // Ensure it hits generating state
  await expect(page.getByText('AI Architect')).toBeVisible();
});
