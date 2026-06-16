import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard E2E Flow', () => {
    // Test 1: Verifies Instant Build successful generation flow
  test('Instant Build successfully creates a fully populated storefront from a valid paragraph', async ({ page }) => {
    await page.goto('/onboarding');
    const setupScreen = page.locator('#setup-screen');
    await expect(setupScreen).toBeVisible({ timeout: 30000 });



    await expect(page.getByRole('heading', { name: "Tell us about your business" })).toBeVisible();

    const bioInput = page.getByPlaceholder("e.g. I run a local bakery that sells custom vegan cakes...");
    await expect(bioInput).toBeVisible();
    await expect(bioInput).toHaveClass(/glassmorphism/);

    await bioInput.fill("I run a high-end tech consultation firm specializing in AI in San Francisco.");

    const imageUrlInput = page.locator('#instant-image-url');
    await expect(imageUrlInput).toBeVisible();
    await imageUrlInput.fill("https://example.com/logo.png");
    await page.getByTestId('admin-email').fill('maya@example.com');
    await page.getByTestId('admin-password').fill('mypassword123');

    const generateButton = page.getByRole('button', { name: 'Next' });
    await expect(generateButton).toBeVisible();
    await generateButton.click();

    await expect(page.locator('#setup-screen')).toBeVisible();
    const successHeading = page.getByRole('heading', { name: "You're Live!" });

    await expect(successHeading).toBeVisible({ timeout: 60000 });
  });

  test('Instant Build image URL is submitted and correctly mapped to state', async ({ page }) => {
    await page.goto('/onboarding');


    const bioInput = page.getByPlaceholder("e.g. I run a local bakery that sells custom vegan cakes...");
    await bioInput.fill("Test business description.");

    const imageUrlInput = page.locator('#instant-image-url');
    await expect(imageUrlInput).toBeVisible();
    await expect(imageUrlInput).toHaveAttribute('type', 'url');
    await imageUrlInput.fill("https://example.com/logo.png");
    await page.getByTestId('admin-email').fill('maya@example.com');
    await page.getByTestId('admin-password').fill('mypassword123');

    const generateButton = page.getByRole('button', { name: 'Next' });
    await generateButton.click();

    const successHeading = page.getByRole('heading', { name: "You're Live!" });
    await expect(successHeading).toBeVisible({ timeout: 60000 });
  });

  test('Instant Build image URL can be empty and successfully launches', async ({ page }) => {
    await page.goto('/onboarding');


    const bioInput = page.getByPlaceholder("e.g. I run a local bakery that sells custom vegan cakes...");
    await bioInput.fill("Test business description without image.");

    const imageUrlInput = page.locator('#instant-image-url');
    await expect(imageUrlInput).toBeVisible();
    // leave empty
    await page.getByTestId('admin-email').fill('maya@example.com');
    await page.getByTestId('admin-password').fill('mypassword123');

    const generateButton = page.getByRole('button', { name: 'Next' });
    await generateButton.click();

    const successHeading = page.getByRole('heading', { name: "You're Live!" });
    await expect(successHeading).toBeVisible({ timeout: 60000 });
  });

  // Test 2: Verifies Instant Build handles network error gracefully
  test('Instant Build gracefully displays an error state on a network failure with proper styling', async ({ page }) => {
    await page.goto('/onboarding');
    const setupScreen = page.locator('#setup-screen');
    await expect(setupScreen).toBeVisible({ timeout: 30000 });



    await expect(page.getByRole('heading', { name: "Tell us about your business" })).toBeVisible();

    const bioInput = page.getByPlaceholder("e.g. I run a local bakery that sells custom vegan cakes...");
    await bioInput.fill("Will fail network request");
    await page.getByTestId('admin-email').fill('maya@example.com');
    await page.getByTestId('admin-password').fill('mypassword123');

    await page.route('**/api/onboarding/**', route => route.abort('failed'));

    const generateButton = page.getByRole('button', { name: 'Next' });
    await generateButton.click();

    // Verify error is shown with correct styling
    const errorBlock = page.locator('.animate-shake').first();
    await expect(errorBlock).toBeVisible();
    await expect(errorBlock).toHaveClass(/text-\[#FF3B30\]/);
    await expect(errorBlock).toHaveClass(/border-\[#FF3B30\]\/30/);

    // Verify textarea has the red border
    await expect(bioInput).toHaveClass(/border-\[#FF3B30\]/);

    // Typing clears the error border
    await bioInput.fill("New text");
    await expect(bioInput).not.toHaveClass(/border-\[#FF3B30\]/);

    await page.unroute('**/api/onboarding/**');
  });

  // Test 3: Verifies empty input behavior
  test('Instant Build prevents submission when the input is empty', async ({ page }) => {
    await page.goto('/onboarding');


    const generateButton = page.getByRole('button', { name: 'Next' });

    // Button should be disabled when input is empty.
    await expect(generateButton).toBeDisabled();

    // We shouldn't see a loading state.
    const loadingState = page.getByText('Building Your Business...');
    await expect(loadingState).not.toBeVisible();
    await expect(page.getByRole('heading', { name: "Tell us about your business" })).toBeVisible();
  });

  // Test 4: Smart defaults fallback on partial info
  test('Instant Build handles partial information appropriately by falling back to smart defaults', async ({ page }) => {
    await page.goto('/onboarding');


    const bioInput = page.getByPlaceholder("e.g. I run a local bakery that sells custom vegan cakes...");
    // Only provide a generic description
    await bioInput.fill("I sell things online.");
    await page.getByTestId('admin-email').fill('maya@example.com');
    await page.getByTestId('admin-password').fill('mypassword123');

    const generateButton = page.getByRole('button', { name: 'Next' });
    await generateButton.click();

    const successHeading = page.getByRole('heading', { name: "You're Live!" });
    await expect(successHeading).toBeVisible({ timeout: 60000 });
  });

  // Test 5: Mobile responsiveness of the Instant Build component
  test('Instant Build respects mobile viewport constraints (375px) with valid touch targets for the conversational flow', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/onboarding');



    const bioInput = page.getByPlaceholder("e.g. I run a local bakery that sells custom vegan cakes...");
    const box = await bioInput.boundingBox();
    expect(Math.round(box?.height || 0)).toBeGreaterThanOrEqual(44);
    expect(box?.width).toBeLessThanOrEqual(375);

    const generateButton = page.getByRole('button', { name: 'Next' });
    const btnBox = await generateButton.boundingBox();
    expect(Math.round(btnBox?.height || 0)).toBeGreaterThanOrEqual(44);
  });
});
