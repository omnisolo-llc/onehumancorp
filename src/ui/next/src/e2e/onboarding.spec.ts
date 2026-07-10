import { test, expect } from '@playwright/test';

test.describe('OnboardingFlow', () => {

  test('Maya the Home Baker sets up her store using the manual Wizard', async ({ page }) => {
    await page.goto('/onboarding');

    // The default is now Zero-Click mode, so switch to wizard
    await page.getByRole('button', { name: 'Use step-by-step wizard instead' }).click();

    await expect(page.getByText("Setup Assistant")).toBeVisible();
    await page.getByRole('button', { name: 'Start My Business' }).click();

    await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Maya Bakery');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Custom vegan cakes');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder(/Portland, OR/i).fill('Austin, TX');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder(/Local families, Tech startups/i).fill('Local families');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.waitForSelector("text=Review Details", { state: "visible", timeout: 30000 });
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByText('Classic').click();
    await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Maya');
    await page.getByPlaceholder(/you@example.com/i).fill('maya@example.com');
    await page.getByPlaceholder(/••••••••/i).fill('password123');

    await page.getByRole('button', { name: 'Approve & Publish' }).click();
    await expect(page.getByText("You're Live!")).toBeVisible();
  });

  test('Maya sets up her store using Zero-Click Onboarding', async ({ page }) => {
    await page.goto('/onboarding');

    await expect(page.getByText("Zero-Click Launch")).toBeVisible();

    await page.getByPlaceholder(/e.g. I sell custom vegan cupcakes/i).fill('I bake custom vegan cakes in Austin TX');
    await page.getByRole('button', { name: 'Generate Storefront' }).click();

    await expect(page.getByText("Generating Your Storefront...")).toBeVisible();
    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 15000 });
  });
});
