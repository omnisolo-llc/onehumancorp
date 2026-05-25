import { test, expect } from './fixtures';

test.describe('Onboarding Wizard', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Maya (The Home Baker) onboarding flow', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('maya@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    await page.goto('/onboarding');

    // Step 1: Category
    await expect(page.getByRole('heading', { name: "What do you do?" })).toBeVisible();
    await page.getByRole('button', { name: /Bake/i }).click();

    // Step 2: Name
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Maya's Cakes");

    // Check Glassmorphism aesthetic on input
    const inputField = page.getByPlaceholder("e.g. Maya's Cakes");
    await expect(inputField).toHaveClass(/backdrop-blur-\[20px\]/);

    await page.getByRole('button', { name: /Next/i }).click();

    // Step 3: Generating/Loading screen, then redirect to Dashboard
    await expect(page.getByRole('heading', { name: 'Generating your store...' })).toBeVisible({ timeout: 15000 });

    // Wait for Dashboard
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=1 Action Required: Connect Stripe to accept payments.')).toBeVisible();
  });

  test('Carlos (Handyman) onboarding flow', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('carlos@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    await page.goto('/onboarding');

    // Step 1: Category
    await expect(page.getByRole('heading', { name: "What do you do?" })).toBeVisible();
    await page.getByRole('button', { name: /Fix/i }).click();

    // Step 2: Name
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Carlos Plumbing");
    await page.getByRole('button', { name: /Next/i }).click();

    // Step 3: Generating/Loading screen, then redirect to Dashboard
    await expect(page.getByRole('heading', { name: 'Generating your store...' })).toBeVisible({ timeout: 15000 });
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
  });

  test('Priya (Boutique Owner) onboarding flow using text input', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('priya@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    await page.goto('/onboarding');

    // Step 1: Category using text input
    await expect(page.getByRole('heading', { name: "What do you do?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Sell custom cakes, plumbing").fill("Boutique clothing store");
    await page.getByRole('button', { name: /Next/i }).click();

    // Step 2: Name
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Priya's Boutique");
    await page.getByRole('button', { name: /Next/i }).click();

    // Step 3: Generating/Loading screen, then redirect to Dashboard
    await expect(page.getByRole('heading', { name: 'Generating your store...' })).toBeVisible({ timeout: 15000 });
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
  });

  test('Leo (Music Tutor) onboarding flow using Teach category', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('leo@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    await page.goto('/onboarding');

    // Step 1: Category
    await expect(page.getByRole('heading', { name: "What do you do?" })).toBeVisible();
    await page.getByRole('button', { name: /Teach/i }).click();

    // Step 2: Name
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Leo's Music Tutoring");
    await page.getByRole('button', { name: /Next/i }).click();

    // Step 3: Generating/Loading screen, then redirect to Dashboard
    await expect(page.getByRole('heading', { name: 'Generating your store...' })).toBeVisible({ timeout: 15000 });
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
  });

  test('Fatima (Food Cart) onboarding flow using text input', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill('fatima@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    await page.goto('/onboarding');

    // Step 1: Category using text input
    await expect(page.getByRole('heading', { name: "What do you do?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Sell custom cakes, plumbing").fill("Halal Food Cart");
    await page.getByRole('button', { name: /Next/i }).click();

    // Step 2: Name
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Fatima's Halal Cart");
    await page.getByRole('button', { name: /Next/i }).click();

    // Step 3: Generating/Loading screen, then redirect to Dashboard
    await expect(page.getByRole('heading', { name: 'Generating your store...' })).toBeVisible({ timeout: 15000 });
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
  });
});
