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

    // 1. Acquisition & Onboarding start
    await page.goto('/onboarding');

    // Wait for the Smart Builder welcome screen
    await expect(page.getByRole('heading', { name: 'OneHumanCorp' })).toBeVisible();
    await page.getByRole('button', { name: 'Start a Business' }).click();

    // Wait for the Bio input screen
    await expect(page.getByRole('heading', { name: 'Welcome to OHC Smart Builder' })).toBeVisible();

    // Fill in the single text block input
    await page.getByLabel('Business Bio').fill("I bake custom vegan cakes in Seattle. Maya's Cakes.");

    // Setup network interception to catch the API requests
    const intakeRequestPromise = page.waitForRequest(request =>
      request.url().includes('/api/onboarding/intake') && request.method() === 'POST'
    );
    const startRequestPromise = page.waitForRequest(request =>
      request.url().includes('/api/onboarding/start') && request.method() === 'POST'
    );

    // Click Build My Storefront
    await page.getByRole('button', { name: 'Build My Storefront' }).click();

    // Verify the intake request was sent with the description
    const intakeRequest = await intakeRequestPromise;
    const intakeData = JSON.parse(intakeRequest.postData() || '{}');
    expect(intakeData.description).toBe("I bake custom vegan cakes in Seattle. Maya's Cakes.");

    // Verify the start request was sent
    const startRequest = await startRequestPromise;
    const startData = JSON.parse(startRequest.postData() || '{}');
    expect(startData.bio).toBe("I bake custom vegan cakes in Seattle. Maya's Cakes.");
    // Assert other fields that should be extrapolated or defaulted
    expect(startData.company_name).toBeDefined();
    expect(startData.business_type).toBeDefined();

    // 2. Simplified Mobile First Onboarding - wait for it to generate
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
    await expect(page.getByText('Storefront Generated!')).toBeVisible();

    // 3. Preview and Launch
    await page.getByRole('button', { name: 'Preview Site' }).click();
    await expect(page.getByText('Preview Mode')).toBeVisible();

    await page.getByRole('button', { name: '1-Tap Launch' }).click();

    // 4. Activation
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });

    // 5. Verify Dashboard redirect
    await page.getByRole('button', { name: 'Go to Dashboard' }).click();
    await expect(page.getByText('Agent Updates')).toBeVisible({ timeout: 15000 });
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

    // Wait for the Smart Builder welcome screen
    await expect(page.getByRole('heading', { name: 'OneHumanCorp' })).toBeVisible();
    await page.getByRole('button', { name: 'Start a Business' }).click();

    // Wait for the Bio input screen
    await expect(page.getByRole('heading', { name: 'Welcome to OHC Smart Builder' })).toBeVisible();

    // Fill in the single text block input
    await page.getByLabel('Business Bio').fill("I am Carlos and I do plumbing and fix pipes. Carlos Plumbing.");

    // Setup network interception to catch the API requests
    const startRequestPromise = page.waitForRequest(request =>
      request.url().includes('/api/onboarding/start') && request.method() === 'POST'
    );

    // Click Build My Storefront
    await page.getByRole('button', { name: 'Build My Storefront' }).click();

    // Verify the start request was sent
    await startRequestPromise;

    // 2. Simplified Mobile First Onboarding - wait for it to generate
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
    await expect(page.getByText('Storefront Generated!')).toBeVisible();

    // 3. Preview and Launch
    await page.getByRole('button', { name: 'Preview Site' }).click();
    await expect(page.getByText('Preview Mode')).toBeVisible();

    await page.getByRole('button', { name: '1-Tap Launch' }).click();

    // 4. Activation
    await expect(page.getByRole('heading', { name: "You're Live!" })).toBeVisible({ timeout: 15000 });

    // 5. Verify Dashboard redirect
    await page.getByRole('button', { name: 'Go to Dashboard' }).click();
    await expect(page.getByText('Agent Updates')).toBeVisible({ timeout: 15000 });
  });
});
