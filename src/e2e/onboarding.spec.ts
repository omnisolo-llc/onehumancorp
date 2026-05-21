import { test, expect } from './fixtures';

test.describe('Onboarding Wizard', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Maya (The Home Baker) onboarding flow', async ({ page }) => {
    await page.goto('/onboarding');

    // Step 1: Initial config
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Maya's Cakes");
    await page.getByPlaceholder("e.g. I bake custom vegan cakes.").fill("I bake custom vegan cakes in Seattle.");
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: "What do you sell?" })).toBeVisible();
    await page.getByRole('button', { name: 'Food / Custom Cakes' }).click();

    await expect(page.getByRole('heading', { name: "How do you want to get paid?" })).toBeVisible();
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: "Connect Instagram?" })).toBeVisible();
    await page.getByRole('button', { name: 'Connect IG' }).click();

    await expect(page.getByRole('heading', { name: "Set Prices & Deposit Rules" })).toBeVisible();
    await page.getByRole('button', { name: 'Publish Store' }).click();

    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });
  });

  test('Carlos (The Handyman) onboarding flow', async ({ page }) => {
    await page.goto('/onboarding');

    // Step 1: Initial config
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Carlos' Handyman Services");
    await page.getByPlaceholder("e.g. I bake custom vegan cakes.").fill("I fix pipes and paint walls.");
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: "What do you sell?" })).toBeVisible();
    await page.getByRole('button', { name: 'Services / Bookings' }).click();

    await expect(page.getByRole('heading', { name: "What services do you offer?" })).toBeVisible();
    await page.getByPlaceholder("Plumbing, Painting").fill("Plumbing, Painting");
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: "Set Working Hours & Deposits" })).toBeVisible();
    await page.getByRole('button', { name: 'Publish Store' }).click();

    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });
  });

  test('Priya (The Boutique Owner) onboarding flow', async ({ page }) => {
    await page.goto('/onboarding');

    // Step 1: Initial config
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Priya's Boutique");
    await page.getByPlaceholder("e.g. I bake custom vegan cakes.").fill("I sell trendy clothing.");
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: "What do you sell?" })).toBeVisible();
    await page.getByRole('button', { name: 'Physical Products' }).click();

    await expect(page.getByRole('heading', { name: "Upload Inventory" })).toBeVisible();
    await page.getByRole('button', { name: 'Upload CSV' }).click();

    await expect(page.getByRole('heading', { name: "Inventory Uploaded" })).toBeVisible();
    await page.getByRole('button', { name: 'Publish Store' }).click();

    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });
  });

  test('Leo (The Music Tutor) onboarding flow', async ({ page }) => {
    await page.goto('/onboarding');

    // Step 1: Initial config
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Leo's Music");
    await page.getByPlaceholder("e.g. I bake custom vegan cakes.").fill("I teach music.");
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: "What do you sell?" })).toBeVisible();
    await page.getByRole('button', { name: 'Services & Subscriptions' }).click();

    await expect(page.getByRole('heading', { name: "Connect Calendar?" })).toBeVisible();
    await page.getByRole('button', { name: 'Connect Google Calendar' }).click();

    await expect(page.getByRole('heading', { name: "Calendar Synced" })).toBeVisible();
    await page.getByRole('button', { name: 'Publish Store' }).click();

    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });
  });

  test('Fatima (The Food Cart Operator) onboarding flow', async ({ page }) => {
    await page.goto('/onboarding');

    // Step 1: Initial config
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Cakes").fill("Fatima's Food Cart");
    await page.getByPlaceholder("e.g. I bake custom vegan cakes.").fill("I sell falafels.");
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByRole('heading', { name: "What do you sell?" })).toBeVisible();
    await page.getByRole('button', { name: 'Food & Beverage (Cart)' }).click();

    await expect(page.getByRole('heading', { name: "Take photos of menu" })).toBeVisible();
    await page.getByRole('button', { name: 'Upload Photos' }).click();

    await expect(page.getByRole('heading', { name: "Menu Ready" })).toBeVisible();
    await page.getByRole('button', { name: 'Publish Store' }).click();

    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });
  });
});
