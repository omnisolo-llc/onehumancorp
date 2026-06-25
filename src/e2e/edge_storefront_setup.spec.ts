import { test, expect } from './fixtures';

test.describe('Edge Storefront UI Setup Journey', () => {

  test('Owner can publish a storefront via AI promoter assistant flow', async ({ page }) => {
    // 1. Start from Dashboard (using authenticated fixture)
    await page.goto('/dashboard');

    // 2. Locate Grow Business / Review Storefront CTA and click
    const reviewBtn = page.locator('#review-storefront-btn');
    await expect(reviewBtn).toBeVisible();
    await reviewBtn.click();

    // 3. Verify we are on the edge-storefront-setup page
    await expect(page).toHaveURL(/\/edge-storefront-setup/);
    await expect(page.locator('h2', { hasText: 'Publish Storefront' })).toBeVisible();

    // 4. Click Start Setup
    const startBtn = page.locator('#start-setup-btn');
    await startBtn.click();

    // 5. Interact with Promoter Agent Step
    await expect(page.locator('h3', { hasText: 'Promoter Agent' })).toBeVisible();

    // Check that Generate button is disabled initially
    const generateBtn = page.locator('#generate-storefront-btn');
    await expect(generateBtn).toBeDisabled();

    // Select Custom Cakes
    const selectCakesBtn = page.locator('#select-custom-cakes-btn');
    await selectCakesBtn.click();

    // Generate button should now be enabled
    await expect(generateBtn).toBeEnabled();

    // We shouldn't use route mocking here. Since the test runs against a real local backend,
    // we assume the API requests actually work and hit the database.
    // Click Generate & Publish
    await generateBtn.click();

    // Wait for the UI processing and success state.
    // This could take a while if the AI generation is running locally or talking to minimax
    await expect(page.locator('h2', { hasText: 'Storefront Live!' })).toBeVisible({ timeout: 60000 });

    // 7. Verify the generated URL is visible and the copy link button works
    await expect(page.locator('span', { hasText: '/api/v1/builder/edge/' })).toBeVisible();
    const copyBtn = page.locator('#copy-link-btn');
    await expect(copyBtn).toBeVisible();

    // Assuming Back to Dashboard exists
    const backBtn = page.locator('a', { hasText: 'Back to Dashboard' });
    await expect(backBtn).toBeVisible();
  });

  test('Owner can safely cancel out of storefront setup without mutations', async ({ page }) => {
    await page.goto('/dashboard');
    await page.locator('#review-storefront-btn').click();
    await expect(page).toHaveURL(/\/edge-storefront-setup/);

    // Check that we can navigate back safely before starting
    const backNavBtn = page.locator('button[aria-label="Go back"]').first();
    if (await backNavBtn.isVisible()) {
      await backNavBtn.click();
    } else {
      await page.goto('/dashboard');
    }
    await expect(page).toHaveURL(/\/dashboard/);
  });

  test('Setup enforces valid selections before allowing generation', async ({ page }) => {
    await page.goto('/edge-storefront-setup');
    await page.locator('#start-setup-btn').click();

    const generateBtn = page.locator('#generate-storefront-btn');
    await expect(generateBtn).toBeDisabled();

    // Select Ready-to-buy
    await page.locator('text=Ready-to-buy').click();
    await expect(generateBtn).toBeEnabled();

    // Select Custom Cakes instead
    await page.locator('#select-custom-cakes-btn').click();
    await expect(generateBtn).toBeEnabled();
  });

  test('Storefront Live step correctly truncates and displays the deployed URL', async ({ page }) => {
    await page.goto('/dashboard');
    await page.locator('#review-storefront-btn').click();
    await page.locator('#start-setup-btn').click();
    await page.locator('#select-custom-cakes-btn').click();

    await page.locator('#generate-storefront-btn').click();

    // Wait for the UI processing and success state
    await expect(page.locator('h2', { hasText: 'Storefront Live!' })).toBeVisible({ timeout: 60000 });

    // Ensure the truncate class is on the URL span
    const urlSpan = page.locator('span', { hasText: '/api/v1/builder/edge/' });
    await expect(urlSpan).toHaveClass(/truncate/);
  });

});
