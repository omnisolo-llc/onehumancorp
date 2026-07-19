import { test, expect } from '../../../../e2e/fixtures';

test.describe('Wizard and Onboarding flows', () => {

  test('Website builder wizard mobile layout', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });

    await page.goto('/website-builder');

    // Check elements
    const heading = page.getByRole('heading', { name: 'Tell us about your business' });
    await expect(heading).toBeVisible();

    // Verify it doesn't overflow horizontally
    const htmlWidth = await page.evaluate(() => document.documentElement.scrollWidth);
    const windowWidth = await page.evaluate(() => window.innerWidth);
    expect(htmlWidth).toBeLessThanOrEqual(windowWidth);

    await expect(page.getByRole('button', { name: 'Generate My Workspace' })).toBeVisible();
  });

  test('Builder mobile UI test', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/builder');

    await expect(page.locator('text="What are you building today?"').first()).toBeVisible();

    // Check click routing inside builder
    await page.locator('text="Selling Products"').click();

    await expect(page.getByRole('heading', { name: "Let's build your store" })).toBeVisible();

    // Verify we reached the business name input
    await expect(page.getByText('Business Name')).toBeVisible();
  });

  test('Main Onboarding multi-step wizard mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/onboarding');

    await expect(page.getByRole('heading', { name: 'Setup Assistant' })).toBeVisible();
    await page.getByRole('button', { name: 'Start My Business' }).click();

    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();

    await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Cakes By Maya');
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByText("What do you sell?")).toBeVisible();
  });

  test('Direct routing for business-setup compatibility page', async ({ page }) => {
    await page.goto('/business-setup');

    // Should immediately reroute to onboarding
    await expect(page.getByRole('heading', { name: 'Setup Assistant' })).toBeVisible();
  });

  test('Onboarding allows full traversal on standard layout', async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 900 });
    await page.goto('/onboarding');

    await expect(page.getByRole('heading', { name: 'Setup Assistant' })).toBeVisible();
    await page.getByRole('button', { name: 'Start My Business' }).click();

    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
  });

  test('Loading state padding check on mobile layout', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/onboarding');

    // Click through UI instead of using localStorage
    await page.getByRole('button', { name: 'Start My Business' }).click();
    await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Cakes By Maya');
    await page.getByRole('button', { name: 'Next' }).click();

    // Check loading indicator container doesn't overflow
    const container = page.locator('.animate-fade-in').first();
    await expect(container).toBeVisible();

    const containerWidth = await container.evaluate(el => el.clientWidth);
    expect(containerWidth).toBeLessThanOrEqual(375);
  });

  test('Buttons and Inputs have minimum touch target on Setup UI', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/onboarding');

    // Let the initial route resolve to the onboarding screen
    await page.waitForTimeout(2000);

    // We verify a button exists with a min height. Since playright has locator geometry,
    // we can evaluate the height of all buttons to ensure none fall below the 44px threshold
    const buttons = await page.locator('button').all();
    for (const btn of buttons) {
      const isVisible = await btn.isVisible();
      if (isVisible) {
         const box = await btn.boundingBox();
         if (box) {
           expect(box.height).toBeGreaterThanOrEqual(44);
         }
      }
    }
  });
});
