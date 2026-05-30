import { test, expect } from '@playwright/test';

test('Onboarding Flow - Start to Dashboard', async ({ page }) => {
  // Use mock for all API calls since backend is not fully setup
  await page.route('**/api/onboarding/state', route => route.fulfill({ json: { wizardState: {} } }));
  await page.route('**/api/onboarding/intake', route => route.fulfill({
    json: {
        business_type: 'Bakery',
        business_name: 'Maya Bakery',
        categories: ['food'],
        initial_products: [{ name: 'Cake', price: '20' }]
    }
  }));

  await page.goto('http://localhost:3000/onboarding');

  // Verify URL
  await expect(page).toHaveURL(/.*onboarding/);

  // Intake Step 1
  await expect(page.getByText("What's the name of your business?")).toBeVisible();
  await page.getByPlaceholder("e.g. Maya's Custom Cakes").fill('Maya Bakery');
  await page.getByRole('button', { name: 'Next' }).click();

  // Intake Step 2
  await expect(page.getByText("What do you sell?")).toBeVisible();
  await page.getByPlaceholder("e.g. I bake custom vegan cakes").fill('Cakes and Pastries');
  await page.getByRole('button', { name: 'Next' }).click();

  // Intake Step 3
  await expect(page.getByText("Where are you located?")).toBeVisible({ timeout: 10000 });
  await page.getByPlaceholder("e.g. Portland, OR").fill('New York');
  await page.getByRole('button', { name: 'Generate My Business' }).click();

  // Wait to check review details
  await expect(page.getByText('Review Details')).toBeVisible({ timeout: 10000 });
});
