import { expect, test } from './fixtures';

test('Maya (The Home Baker) - Physical Products (Custom Orders)', async ({ page }) => {
  // Start Onboarding
  await page.goto('/onboarding');
  await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();

  // Step 1: Tell OHC about the business
  await page.getByPlaceholder('e.g., custom cakes, plumbing, vintage clothes').fill('custom cakes');
  await page.getByRole('button', { name: 'Continue' }).click();

  // Step 2: The Basics
  await expect(page.getByRole('heading', { name: 'The Basics' })).toBeVisible();
  await page.getByRole('textbox').nth(0).fill('Maya\'s Custom Cakes'); // Business Name

  // Fill first product details
  await page.getByRole('textbox').nth(1).fill('Online Store'); // Business Type
  await page.getByRole('textbox').nth(2).fill('Vegan Chocolate Cake'); // First Product Name
  await page.getByRole('textbox').nth(3).fill('50.00'); // First Product Price

  await page.getByRole('button', { name: 'Continue' }).click();

  // Step 3: Style & Team
  await expect(page.getByRole('heading', { name: 'Style & Team' })).toBeVisible();
  await page.getByText('Modern').click(); // Select Modern template
  await page.getByRole('button', { name: 'Launch Store' }).click();

  // Step 4 & 5: Loading & Live
  await expect(page.getByText('Building Your Business...')).toBeVisible();
  await expect(page.getByText('You\'re Live!')).toBeVisible({ timeout: 15000 });

  // Go to Dashboard
  await page.getByRole('link', { name: 'Go to Dashboard' }).click();
  await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
});

test('Carlos (The Freelance Handyman) - Services & Bookings', async ({ page }) => {
  // Start Onboarding
  await page.goto('/onboarding');
  await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();

  // Step 1: Tell OHC about the business
  await page.getByPlaceholder('e.g., custom cakes, plumbing, vintage clothes').fill('Home Repair Services');
  await page.getByRole('button', { name: 'Continue' }).click();

  // Step 2: The Basics
  await expect(page.getByRole('heading', { name: 'The Basics' })).toBeVisible();
  await page.getByRole('textbox').nth(0).fill('Carlos Handyman Services'); // Business Name

  // Fill first product details
  await page.getByRole('textbox').nth(1).fill('Service Business'); // Business Type
  await page.getByRole('textbox').nth(2).fill('Plumbing Fix'); // First Product Name
  await page.getByRole('textbox').nth(3).fill('100.00'); // First Product Price

  await page.getByRole('button', { name: 'Continue' }).click();

  // Step 3: Style & Team
  await expect(page.getByRole('heading', { name: 'Style & Team' })).toBeVisible();
  await page.getByText('Classic').click(); // Select Classic template
  await page.getByRole('button', { name: 'Launch Store' }).click();

  // Step 4 & 5: Loading & Live
  await expect(page.getByText('Building Your Business...')).toBeVisible();
  await expect(page.getByText('You\'re Live!')).toBeVisible({ timeout: 15000 });

  // Go to Dashboard
  await page.getByRole('link', { name: 'Go to Dashboard' }).click();
  await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
});

test('Fatima (The Food Cart Operator) - Pre-orders', async ({ page }) => {
  // Start Onboarding
  await page.goto('/onboarding');
  await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();

  // Step 1: Tell OHC about the business
  await page.getByPlaceholder('e.g., custom cakes, plumbing, vintage clothes').fill('Halal Food Cart');
  await page.getByRole('button', { name: 'Continue' }).click();

  // Step 2: The Basics
  await expect(page.getByRole('heading', { name: 'The Basics' })).toBeVisible();
  await page.getByRole('textbox').nth(0).fill('Fatima\'s Halal Cart'); // Business Name

  // Fill first product details
  await page.getByRole('textbox').nth(1).fill('Food Cart'); // Business Type
  await page.getByRole('textbox').nth(2).fill('Chicken over Rice'); // First Product Name
  await page.getByRole('textbox').nth(3).fill('12.00'); // First Product Price

  await page.getByRole('button', { name: 'Continue' }).click();

  // Step 3: Style & Team
  await expect(page.getByRole('heading', { name: 'Style & Team' })).toBeVisible();
  await page.getByText('Bold').click(); // Select Bold template
  await page.getByRole('button', { name: 'Launch Store' }).click();

  // Step 4 & 5: Loading & Live
  await expect(page.getByText('Building Your Business...')).toBeVisible();
  await expect(page.getByText('You\'re Live!')).toBeVisible({ timeout: 15000 });

  // Go to Dashboard
  await page.getByRole('link', { name: 'Go to Dashboard' }).click();
  await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
});
