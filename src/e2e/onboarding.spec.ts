import { test, expect } from './fixtures';

test.describe('Onboarding Wizard', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Maya (The Home Baker) onboarding flow', async ({ page }) => {
    await page.route('**/api/onboarding/intake', async route => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ business_name: "Maya's Cakes", business_type: "Bakery", categories: ["food", "physical"], initial_products: [{ name: "Custom Vegan Cake", price: "45.00" }] }) }));
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

    // Check chips
    await expect(page.getByRole('button', { name: 'Online Store' })).toBeVisible();

    // Click a preset chip instead of filling
    await page.getByRole('button', { name: 'Online Store' }).click();

    // Step 2
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    // Fill in the business name
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Maya's Cakes");

    // Click Next
    await page.getByRole('button', { name: /Next/i }).click();

    // Step 3
    await expect(page.getByRole('heading', { name: "What's your niche?" })).toBeVisible();

    // Fill in the niche - test clicking a chip
    await expect(page.getByRole('button', { name: 'Food & Beverage' })).toBeVisible();
    await page.getByRole('button', { name: 'Food & Beverage' }).click();

    // 2. Simplified Mobile First Onboarding - wait for it to generate
    await expect(page.getByRole('heading', { name: 'Ready to Launch!' })).toBeVisible({ timeout: 15000 });

    // Verify keyboard optimizations for price input
    const priceInput = page.getByPlaceholder('0.00');
    await expect(priceInput).toHaveAttribute('inputMode', 'decimal');
    await expect(priceInput).toHaveAttribute('pattern', '[0-9]*\\.?[0-9]*');

    // Verify glassmorphism aesthetics applied
    await expect(priceInput).toHaveClass(/backdrop-blur/);

    // Configure products and domain before publishing
    await page.getByRole('button', { name: 'Playful' }).click();
    await page.getByRole('button', { name: /Connect Custom Domain/i }).click();

    // Publish
    await page.getByRole('button', { name: /Publish Now/i }).click();

    // 3. Activation
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });

    // 4. Verify Dashboard redirect and action banner
    await page.getByRole('link', { name: /Go to Dashboard/i }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    // Handle either case since the mock data might change
    const stripeBanner = page.locator('text=1 Action Required: Connect Stripe to accept payments.');
    const setupBanner = page.locator('text=Complete Stripe Setup');

    await expect(stripeBanner.or(setupBanner)).toBeVisible({ timeout: 15000 });
  });

  test('Carlos (Handyman) onboarding flow', async ({ page }) => {
    await page.route('**/api/onboarding/intake', async route => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ business_name: "Carlos Plumbing", business_type: "Service", categories: ["service"], initial_products: [{ name: "Pipe Fix", price: "80.00" }] }) }));
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

    // Click a preset chip
    await page.getByRole('button', { name: 'Service Business' }).click();

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
    await expect(page.getByRole('heading', { name: 'Ready to Launch!' })).toBeVisible({ timeout: 15000 });

    // Verify keyboard optimizations for price input
    const priceInput = page.getByPlaceholder('0.00');
    await expect(priceInput).toHaveAttribute('inputMode', 'decimal');

    // Configure products and domain before publishing
    await page.getByRole('button', { name: 'Modern' }).click();

    // Publish
    await page.getByRole('button', { name: /Publish Now/i }).click();

    // 3. Activation
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });

    // 4. Verify Dashboard redirect and action banner
    await page.getByRole('link', { name: /Go to Dashboard/i }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    // Handle either case since the mock data might change
    const stripeBanner = page.locator('text=1 Action Required: Connect Stripe to accept payments.');
    const setupBanner = page.locator('text=Complete Stripe Setup');

    await expect(stripeBanner.or(setupBanner)).toBeVisible({ timeout: 15000 });
  });
});

  test('Priya (Boutique) onboarding flow', async ({ page }) => {
    await page.route('**/api/onboarding/intake', async route => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ business_name: "Priya's Boutique", business_type: "Retail", categories: ["apparel", "physical"], initial_products: [{ name: "Summer Dress", price: "65.00" }] }) }));
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('priya@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
    await page.goto('/onboarding');
    await expect(page.getByRole('heading', { name: "What do you do?" })).toBeVisible();
    await page.getByRole('button', { name: 'Online Store' }).click();
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Priya's Boutique");
    await page.getByRole('button', { name: /Next/i }).click();
    await expect(page.getByRole('heading', { name: "What's your niche?" })).toBeVisible();
    await page.getByPlaceholder("e.g. I bake custom wedding cakes").fill("I sell trendy summer dresses");
    await page.getByRole('button', { name: /Generate Draft/i }).click();
    await expect(page.getByRole('heading', { name: 'Ready to Launch!' })).toBeVisible({ timeout: 15000 });
    const priceInput = page.getByPlaceholder('0.00');
    await expect(priceInput).toHaveAttribute('inputMode', 'decimal');
    await page.getByRole('button', { name: 'Elegant' }).click();
    // Test advanced options toggle
    await page.getByRole('button', { name: /Advanced Options/i }).click();
    await page.getByRole('button', { name: /Connect Custom Domain/i }).click();
    await page.getByRole('button', { name: /Publish Now/i }).click();
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });
    await page.getByRole('link', { name: /Go to Dashboard/i }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
  });

  test('Leo (Music Tutor) onboarding flow', async ({ page }) => {
    await page.route('**/api/onboarding/intake', async route => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ business_name: "Leo's Music Lessons", business_type: "Service", categories: ["service", "digital"], initial_products: [{ name: "Piano Lesson", price: "50.00" }] }) }));
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('leo@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
    await page.goto('/onboarding');
    await expect(page.getByRole('heading', { name: "What do you do?" })).toBeVisible();
    await page.getByRole('button', { name: 'Service Business' }).click();
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Leo's Music Lessons");
    await page.getByRole('button', { name: /Next/i }).click();
    await expect(page.getByRole('heading', { name: "What's your niche?" })).toBeVisible();
    await page.getByPlaceholder("e.g. I bake custom wedding cakes").fill("I teach piano to beginners");
    await page.getByRole('button', { name: /Generate Draft/i }).click();
    await expect(page.getByRole('heading', { name: 'Ready to Launch!' })).toBeVisible({ timeout: 15000 });
    const priceInput = page.getByPlaceholder('0.00');
    await expect(priceInput).toHaveAttribute('inputMode', 'decimal');
    await page.getByRole('button', { name: 'Minimal' }).click();
    await page.getByRole('button', { name: /Publish Now/i }).click();
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });
    await page.getByRole('link', { name: /Go to Dashboard/i }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
  });

  test('Fatima (Food Cart) onboarding flow', async ({ page }) => {
    await page.route('**/api/onboarding/intake', async route => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ business_name: "Fatima's Falafel", business_type: "Restaurant", categories: ["food"], initial_products: [{ name: "Falafel Wrap", price: "8.50" }] }) }));
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('fatima@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
    await page.goto('/onboarding');
    await expect(page.getByRole('heading', { name: "What do you do?" })).toBeVisible();
    await page.getByRole('button', { name: 'Local Restaurant' }).click();
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Fatima's Falafel");
    await page.getByRole('button', { name: /Next/i }).click();
    await expect(page.getByRole('heading', { name: "What's your niche?" })).toBeVisible();
    await page.getByRole('button', { name: 'Food & Beverage' }).click();
    await expect(page.getByRole('heading', { name: 'Ready to Launch!' })).toBeVisible({ timeout: 15000 });
    const priceInput = page.getByPlaceholder('0.00');
    await expect(priceInput).toHaveAttribute('inputMode', 'decimal');
    await page.getByRole('button', { name: 'Playful' }).click();
    await page.getByRole('button', { name: /Publish Now/i }).click();
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });
    await page.getByRole('link', { name: /Go to Dashboard/i }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
  });
