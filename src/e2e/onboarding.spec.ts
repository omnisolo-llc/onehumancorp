import { test, expect } from './fixtures';

test.describe('Onboarding Wizard', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test.beforeEach(async ({ page }, testInfo) => {
    let email = 'maya@example.com';
    if (testInfo.title.includes('Carlos')) email = 'carlos@example.com';
    if (testInfo.title.includes('Priya')) email = 'priya@example.com';
    if (testInfo.title.includes('Leo')) email = 'leo@example.com';
    if (testInfo.title.includes('Fatima')) email = 'fatima@example.com';

    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill(email);
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
    await page.goto('/onboarding');
  });

  test('Maya (The Home Baker) onboarding flow', async ({ page }) => {
    // Wait for the Smart Builder welcome screen (Step 1)
    await expect(page.getByRole('heading', { name: "What do you do?" })).toBeVisible();

    // Fill in the business type
    await page.getByPlaceholder("e.g. Sell cakes, plumbing").fill("Sell custom cakes");

    // Click Next
    await page.getByRole('button', { name: /Next/i }).click();

    // Step 2
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    // Fill in the business name
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Maya's Cakes");

    // Click Next
    await page.getByRole('button', { name: /Next/i }).click();

    // Step 3
    await expect(page.getByRole('heading', { name: "What's your niche?" })).toBeVisible();

    // Fill in the niche
    await page.getByPlaceholder("e.g. I bake custom wedding cakes").fill("I bake custom vegan cakes");

    // Click Generate Draft
    await page.getByRole('button', { name: /Generate Draft/i }).click();

    // 2. Simplified Mobile First Onboarding - wait for it to generate
    await expect(page.getByRole('heading', { name: 'Looks Great!' })).toBeVisible({ timeout: 15000 });

    // Publish
    await page.getByRole('button', { name: /Publish Now/i }).click();

    // 3. Activation
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });

    // 4. Verify Dashboard redirect and action banner
    await page.getByRole('link', { name: /Go to Dashboard/i }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=1 Action Required: Connect Stripe to accept payments.')).toBeVisible();
  });

  test('Carlos (Handyman) onboarding flow', async ({ page }) => {
    // 0. Start from UI Login
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('carlos@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    // Wait for Dashboard
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    // 1. Acquisition & Onboarding start
    await page.goto('/onboarding');

    // Wait for the Smart Builder welcome screen (Step 1)
    await expect(page.getByRole('heading', { name: "What do you do?" })).toBeVisible();

    // Fill in the business type
    await page.getByPlaceholder("e.g. Sell cakes, plumbing").fill("Plumbing");

    // Click Next
    await page.getByRole('button', { name: /Next/i }).click();

    // Step 2
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    // Fill in the business name
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Carlos Plumbing");

    // Click Next
    await page.getByRole('button', { name: /Next/i }).click();

    // Step 3
    await expect(page.getByRole('heading', { name: "What's your niche?" })).toBeVisible();

    // Fill in the niche
    await page.getByPlaceholder("e.g. I bake custom wedding cakes").fill("I fix pipes and leaks");

    // Click Generate Draft
    await page.getByRole('button', { name: /Generate Draft/i }).click();

    // 2. Simplified Mobile First Onboarding - wait for it to generate
    await expect(page.getByRole('heading', { name: 'Looks Great!' })).toBeVisible({ timeout: 15000 });

    // Publish
    await page.getByRole('button', { name: /Publish Now/i }).click();

    // 3. Activation
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });

    // 4. Verify Dashboard redirect and action banner
    await page.getByRole('link', { name: /Go to Dashboard/i }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=1 Action Required: Connect Stripe to accept payments.')).toBeVisible();
  });

  test('Carlos (The Handyman) onboarding flow', async ({ page }) => {
    await expect(page.getByRole('heading', { name: "What do you do?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Sell cakes, plumbing").fill("Home repairs and plumbing");
    await page.getByRole('button', { name: /Next/i }).click();

    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Carlos Handyman");
    await page.getByRole('button', { name: /Next/i }).click();

    await expect(page.getByRole('heading', { name: "What's your niche?" })).toBeVisible();
    await page.getByPlaceholder("e.g. I bake custom wedding cakes").fill("I do plumbing and home repairs");

    // Intercept API call to return mock data for Carlos
    await page.route('**/api/onboarding/intake', async (route) => {
      await route.fulfill({
        status: 200,
        json: {
          business_name: "Carlos Handyman",
          business_type: "Service",
          initial_products: [{ name: "Plumbing Fix", price: 80 }]
        }
      });
    });
    await page.getByRole('button', { name: /Generate Draft/i }).click();

    await expect(page.getByRole('heading', { name: 'Looks Great!' })).toBeVisible();
    await expect(page.getByText('Carlos Handyman')).toBeVisible();
    await expect(page.getByText('Service')).toBeVisible();
    await expect(page.locator('text=Plumbing Fix - $80').first()).toBeVisible();
  });

  test('Priya (The Boutique Owner) onboarding flow', async ({ page }) => {
    await expect(page.getByRole('heading', { name: "What do you do?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Sell cakes, plumbing").fill("Sell boutique clothing");
    await page.getByRole('button', { name: /Next/i }).click();

    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Priya's Boutique");
    await page.getByRole('button', { name: /Next/i }).click();

    await expect(page.getByRole('heading', { name: "What's your niche?" })).toBeVisible();
    await page.getByPlaceholder("e.g. I bake custom wedding cakes").fill("I sell women's dresses and accessories");

    // Intercept API call
    await page.route('**/api/onboarding/intake', async (route) => {
      await route.fulfill({
        status: 200,
        json: {
          business_name: "Priya's Boutique",
          business_type: "Retail",
          initial_products: [{ name: "Summer Dress", price: 120 }]
        }
      });
    });
    await page.getByRole('button', { name: /Generate Draft/i }).click();

    await expect(page.getByRole('heading', { name: 'Looks Great!' })).toBeVisible();
    await expect(page.getByText("Priya's Boutique")).toBeVisible();
  });

  test('Leo (The Music Tutor) onboarding flow', async ({ page }) => {
    await expect(page.getByRole('heading', { name: "What do you do?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Sell cakes, plumbing").fill("Teach guitar");
    await page.getByRole('button', { name: /Next/i }).click();

    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Leo's Guitar Lessons");
    await page.getByRole('button', { name: /Next/i }).click();

    await expect(page.getByRole('heading', { name: "What's your niche?" })).toBeVisible();
    await page.getByPlaceholder("e.g. I bake custom wedding cakes").fill("I teach acoustic guitar online");

    await page.route('**/api/onboarding/intake', async (route) => {
      await route.fulfill({
        status: 200,
        json: {
          business_name: "Leo's Guitar Lessons",
          business_type: "Education",
          initial_products: [{ name: "1 Hour Lesson", price: 40 }]
        }
      });
    });
    await page.getByRole('button', { name: /Generate Draft/i }).click();
    await expect(page.getByRole('heading', { name: 'Looks Great!' })).toBeVisible();
  });

  test('Fatima (The Food Cart) onboarding flow', async ({ page }) => {
    await expect(page.getByRole('heading', { name: "What do you do?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Sell cakes, plumbing").fill("Halal food cart");
    await page.getByRole('button', { name: /Next/i }).click();

    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Fatima's Food Cart");
    await page.getByRole('button', { name: /Next/i }).click();

    await expect(page.getByRole('heading', { name: "What's your niche?" })).toBeVisible();
    await page.getByPlaceholder("e.g. I bake custom wedding cakes").fill("I sell halal food and drinks");

    await page.route('**/api/onboarding/intake', async (route) => {
      await route.fulfill({
        status: 200,
        json: {
          business_name: "Fatima's Food Cart",
          business_type: "Food & Beverage",
          initial_products: [{ name: "Chicken Over Rice", price: 10 }]
        }
      });
    });
    await page.getByRole('button', { name: /Generate Draft/i }).click();
    await expect(page.getByRole('heading', { name: 'Looks Great!' })).toBeVisible();
  });
});
