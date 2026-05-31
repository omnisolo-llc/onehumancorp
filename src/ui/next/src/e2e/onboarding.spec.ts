import { test, expect } from '@playwright/test';

test('Onboarding flow Domain Choice', async ({ page }) => {
  // We mock backend responses using Playwright's route interception
  await page.route('**/api/onboarding/state', async (route) => {
    await route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({
        wizardState: {}
      })
    });
  });

  await page.route('**/api/onboarding/intake', async (route) => {
    await route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({
        business_type: 'Bakery',
        business_name: 'Maya Bakery',
        categories: ['food'],
        initial_products: [{ name: 'Cake', price: '20' }]
      })
    });
  });

  await page.route('**/api/onboarding/start', async (route) => {
    await route.fulfill({
      status: 200,
      contentType: 'application/json',
      body: JSON.stringify({
        message: 'Your business has been successfully launched.'
      })
    });
  });

  await page.goto('http://localhost:3000/onboarding');

  // step 1: What's the name of your business?
  const nameInput = page.getByPlaceholder(/e.g. Maya's Custom Cakes/i);
  await expect(nameInput).toBeVisible();
  await nameInput.fill('Maya Bakery');
  await page.getByRole('button', { name: /Next/i }).click();

  // step 2: What do you sell?
  const sellInput = page.getByPlaceholder(/I bake custom vegan cakes/i);
  await expect(sellInput).toBeVisible();
  await sellInput.fill('Cakes');
  await page.getByRole('button', { name: /Next/i }).click();

  // step 3: Where are you located?
  const locInput = page.getByPlaceholder(/Portland, OR/i);
  await expect(locInput).toBeVisible();
  await locInput.fill('NY');
  await page.getByRole('button', { name: /Generate My Business/i }).click();

  // Wait for Review Details to show up
  const continueBtn = page.getByRole('button', { name: /Continue/i });
  await expect(continueBtn).toBeVisible({ timeout: 10000 });
  await continueBtn.click();

  // Wait for Style & Team
  await expect(page.getByText('Style & Team')).toBeVisible();

  // Domain Selection
  await expect(page.getByText('Domain Selection')).toBeVisible();

  // Select Custom Domain
  const customDomainBtn = page.getByText('Custom Domain');
  await expect(customDomainBtn).toBeVisible();
  await customDomainBtn.click();

  // Click Launch
  const launchBtn = page.getByRole('button', { name: /Launch Store/i });
  await expect(launchBtn).toBeVisible();
  await launchBtn.click();

  // Wait for success screen
  await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 10000 });
});
