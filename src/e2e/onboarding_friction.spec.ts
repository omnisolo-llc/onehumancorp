import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard UX Tests', () => {
  test('Fatima the Food Cart Operator on a slower network', async ({ page }) => {
    await page.goto('/onboarding');
    await expect(page.getByText("Setup Assistant")).toBeVisible();
    await page.getByRole('button', { name: 'Start My Business' }).click();

    await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Fatima Halal Food');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Halal food cart pickup orders');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder(/Portland, OR/i).fill('New York, NY');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder(/Local families, Tech startups/i).fill('Professionals');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.waitForSelector("text=Review Details", { state: "visible", timeout: 30000 });
    await page.locator('button:has-text("Continue")').click();

    await page.getByText('Bold').click();
    await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Fatima');
    await page.getByPlaceholder(/you@example.com/i).fill('fatima@foodcart.com');
    await page.getByPlaceholder(/••••••••/i).fill('halal123');

    await page.getByRole('button', { name: 'Approve & Publish' }).click();
    await expect(page.getByText("You're Live!")).toBeVisible({ timeout: 15000 });
  });

  test('Submitting empty inputs displays validation errors with visual indicators', async ({ page }) => {
    await page.goto('/onboarding');
    await expect(page.getByText("Setup Assistant")).toBeVisible();
    await page.getByRole('button', { name: 'Start My Business' }).click();

    await expect(page.getByText("What's the name of your business?")).toBeVisible();
    await page.getByPlaceholder(/Maya's Custom Cake/i).fill('  ');
    await page.getByRole('button', { name: 'Next' }).click();

    const businessNameInput = page.getByPlaceholder(/Maya's Custom Cake/i);
    await expect(page.getByText('Business Name must be at least 3 characters.')).toBeVisible();
    await expect(businessNameInput).toHaveClass(/.*border-\[#FF3B30\].*/);

    await businessNameInput.fill('Valid Business Name');
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByText("What do you sell?")).toBeVisible();
    await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('');
    await page.getByRole('button', { name: 'Next' }).click();

    const whatYouSellInput = page.getByPlaceholder(/I bake custom vegan cakes/i);
    await expect(page.getByText('Please tell us what you sell.')).toBeVisible();
    await expect(whatYouSellInput).toHaveClass(/.*border-\[#FF3B30\].*/);
  });
});
