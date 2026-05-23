import { test, expect } from './fixtures';

test.describe('Onboarding Wizard', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Maya (The Home Baker) onboarding flow', async ({ page }) => {
    // 0. Start from UI Login
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('maya@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    // Wait for Dashboard
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    // 1. Acquisition & Onboarding start (assuming a "Start Business Setup" or similar button is on dashboard,
    // or direct navigation if that's the only way from an empty dashboard)
    // For now we'll navigate directly to onboarding after login as a user starting the wizard
    await page.goto('/onboarding');

    // Wait for the Smart Builder welcome screen (Step 1)
    await expect(page.getByRole('heading', { name: "What do you do?" })).toBeVisible();

    // Select the business type
    await page.getByRole('button', { name: 'Physical Products' }).click();

    // Step 2
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    // Fill in the business name
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Maya's Cakes");

    // Click Next
    await page.getByRole('button', { name: /Next/i }).click();

    // Step 3
    await expect(page.getByRole('heading', { name: "What products do you sell?" })).toBeVisible();

    // Fill in the niche
    await page.getByPlaceholder("e.g. Custom vegan cakes, cookies").fill("I bake custom vegan cakes");

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

    // Select the business type
    await page.getByRole('button', { name: 'Services & Bookings' }).click();

    // Step 2
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    // Fill in the business name
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Carlos Plumbing");

    // Click Next
    await page.getByRole('button', { name: /Next/i }).click();

    // Step 3
    await expect(page.getByRole('heading', { name: "What services do you offer?" })).toBeVisible();

    // Fill in the niche
    await page.getByPlaceholder("e.g. Plumbing repairs, pipe fitting").fill("I fix pipes and leaks");

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
  test('Priya (Boutique Owner) onboarding flow', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('priya@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    await page.goto('/onboarding');
    await expect(page.getByRole('heading', { name: "What do you do?" })).toBeVisible();

    await page.getByRole('button', { name: 'Omnichannel Retail' }).click();

    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Priya's Boutique");
    await page.getByRole('button', { name: /Next/i }).click();

    await expect(page.getByRole('heading', { name: "What is your main inventory?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Dresses, shoes, accessories").fill("Dresses, shoes, accessories");
    await page.getByRole('button', { name: /Generate Draft/i }).click();

    await expect(page.getByRole('heading', { name: 'Looks Great!' })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Publish Now/i }).click();

    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });

    await page.getByRole('link', { name: /Go to Dashboard/i }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
  });

  test('Leo (Music Tutor) onboarding flow', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('leo@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    await page.goto('/onboarding');
    await expect(page.getByRole('heading', { name: "What do you do?" })).toBeVisible();

    await page.getByRole('button', { name: 'Digital Services' }).click();

    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Leo's Lessons");
    await page.getByRole('button', { name: /Next/i }).click();

    await expect(page.getByRole('heading', { name: "What digital services do you provide?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Guitar lessons, sheet music").fill("Guitar lessons");
    await page.getByRole('button', { name: /Generate Draft/i }).click();

    await expect(page.getByRole('heading', { name: 'Looks Great!' })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Publish Now/i }).click();

    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });

    await page.getByRole('link', { name: /Go to Dashboard/i }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
  });

  test('Fatima (Food Cart Operator) onboarding flow', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('fatima@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    await page.goto('/onboarding');
    await expect(page.getByRole('heading', { name: "What do you do?" })).toBeVisible();

    await page.getByRole('button', { name: 'Food & Beverage' }).click();

    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Fatima's Falafel");
    await page.getByRole('button', { name: /Next/i }).click();

    await expect(page.getByRole('heading', { name: "What's on your menu?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Falafel, Shawarma, Hummus").fill("Falafel, Shawarma, Hummus");
    await page.getByRole('button', { name: /Generate Draft/i }).click();

    await expect(page.getByRole('heading', { name: 'Looks Great!' })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Publish Now/i }).click();

    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });

    await page.getByRole('link', { name: /Go to Dashboard/i }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
  });
});
