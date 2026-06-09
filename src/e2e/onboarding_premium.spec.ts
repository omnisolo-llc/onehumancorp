import { test, expect } from '@playwright/test';

test.describe('OHC Premium Onboarding Wizard', () => {
  // We use the local python server for these tests since Bazel build is restricted
  const BASE_URL = 'http://localhost:8000';

  test('Maya Persona: Full Onboarding Journey', async ({ page }) => {
    await page.goto(`${BASE_URL}/index.html`);

    // Step 0: Welcome
    await expect(page.getByRole('heading', { name: 'Welcome to OHC' })).toBeVisible();
    await page.getByRole('button', { name: 'Start Onboarding' }).click();

    // Step 1: Business Profile
    await expect(page.getByRole('heading', { name: 'Business Profile' })).toBeVisible();

    // Interaction Audit: Verify name validation
    const nextBtn = page.getByRole('button', { name: 'Next' });
    await nextBtn.click();
    await expect(page.locator('#name-error')).toBeVisible();
    await expect(page.locator('#industry-error')).toBeVisible();

    await page.getByPlaceholder("e.g. Maya's Bakery").fill("Maya's Custom Cakes");
    await page.getByText("🍰 Bakery").click();
    await nextBtn.click();

    // Step 2: AI Assistant Setup
    await expect(page.getByRole('heading', { name: 'Your AI Team' })).toBeVisible();

    // Verify Agent Team display
    await expect(page.getByText('📋 The Manager (Operations)')).toBeVisible();
    await expect(page.getByText('📣 The Promoter (Marketing)')).toBeVisible();

    await page.getByPlaceholder("e.g. Jarvis").fill("Jarvis");
    await page.selectOption("#assistant-tone", "Friendly");
    await page.getByRole('button', { name: 'Finish Setup' }).click();

    // Step 3: Success
    await expect(page.getByRole('heading', { name: "You're all set!" })).toBeVisible();
    await expect(page.locator('#success-msg')).toContainText("Maya's Custom Cakes");
    await expect(page.locator('#success-msg')).toContainText("Jarvis");
  });

  test('Responsive Layout: 375px Verification', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto(`${BASE_URL}/setup.html`);

    const container = page.locator('.container');
    const box = await container.boundingBox();
    // Ensure container doesn't overflow horizontally on 375px
    expect(box?.width).toBeLessThanOrEqual(375);

    // Verify touch targets (inputs and buttons should be at least 48px high in our styles.css)
    const nameInput = page.getByPlaceholder("e.g. Maya's Bakery");
    const inputBtn = await nameInput.boundingBox();
    expect(inputBtn?.height).toBeGreaterThanOrEqual(44);
  });

  test('State Persistence: Refresh during wizard', async ({ page }) => {
    await page.goto(`${BASE_URL}/setup.html`);
    await page.getByPlaceholder("e.g. Maya's Bakery").fill("Persistent Business");
    await page.getByText("🏢 Agency").click();

    // Simulate navigation/save (using localStorage in dev mode)
    await page.getByRole('button', { name: 'Next' }).click();
    await expect(page.getByRole('heading', { name: 'Your AI Team' })).toBeVisible();

    // Go back and refresh
    await page.getByRole('button', { name: 'Back' }).click();
    await page.reload();

    await expect(page.getByPlaceholder("e.g. Maya's Bakery")).toHaveValue("Persistent Business");
    await expect(page.locator('.select-card[data-value="Agency"]')).toHaveClass(/selected/);
  });

  test('Industry Selection Interaction Audit', async ({ page }) => {
    await page.goto(`${BASE_URL}/setup.html`);

    const bakery = page.getByText("🍰 Bakery");
    const agency = page.getByText("🏢 Agency");

    await bakery.click();
    await expect(page.locator('.select-card[data-value="Bakery"]')).toHaveClass(/selected/);
    await expect(page.locator('.select-card[data-value="Agency"]')).not.toHaveClass(/selected/);

    await agency.click();
    await expect(page.locator('.select-card[data-value="Agency"]')).toHaveClass(/selected/);
    await expect(page.locator('.select-card[data-value="Bakery"]')).not.toHaveClass(/selected/);
  });

  test('Dark Mode Visual Audit', async ({ page }) => {
    await page.emulateMedia({ colorScheme: 'dark' });
    await page.goto(`${BASE_URL}/index.html`);

    // Verify background color changes
    const bodyColor = await page.evaluate(() => getComputedStyle(document.body).backgroundColor);
    // rgb(22, 22, 26) is our dark mode background
    expect(bodyColor).toBe('rgb(22, 22, 26)');
  });
});
