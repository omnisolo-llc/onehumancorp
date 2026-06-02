import { test, expect } from '@playwright/test';

test.describe('Onboarding Wizard Flow - Non-Technical Persona', () => {
  test('completes end-to-end onboarding as Maya the Baker', async ({ page }) => {
    // Navigate to the onboarding page natively
    await page.goto('/onboarding');

    // Step 1: Provide Business Name
    await expect(page.locator('text="What\'s the name of your business?"')).toBeVisible({ timeout: 10000 });
    await page.locator('input[placeholder="e.g. Maya\'s Custom Cakes"]').fill("Maya's Custom Cakes");
    await page.locator('button:has-text("Next")').click();

    // Step 2: What do you sell
    await expect(page.locator('text="What do you sell?"')).toBeVisible({ timeout: 5000 });
    await page.locator('textarea[placeholder="e.g. I bake custom vegan cakes for weddings and parties..."]').fill("I bake custom vegan cakes and regular cakes for weddings and parties, focusing on floral designs.");
    await page.locator('button:has-text("Next")').click();

    // Step 3: Location
    await expect(page.locator('text="Where are you located?"')).toBeVisible({ timeout: 5000 });
    await page.locator('input[placeholder="e.g. Portland, OR"]').fill("San Francisco, CA");
    await page.locator('button:has-text("Generate My Business")').click();

    // Verify Review Details step loads
    await expect(page.locator('text="Review Details"')).toBeVisible({ timeout: 30000 });

    // Ensure AI extracted information properly
    await expect(page.locator('input[value="Maya\'s Custom Cakes"]')).toBeVisible();

    // Ensure AI extracted business type is set
    const businessTypeInput = page.locator('label:text("Business Type") + input');
    const businessTypeValue = await businessTypeInput.inputValue();
    expect(businessTypeValue.length).toBeGreaterThan(0);

    // Fill in missing product details if AI didn't catch them
    const productPriceInput = page.locator('label:text("Price") + input');
    if (await productPriceInput.inputValue() === "") {
        await productPriceInput.fill("50.00");
    }

    await page.locator('button:has-text("Continue")').click();

    // Wait for Style & Team step
    await expect(page.locator('text="Style & Team"')).toBeVisible({ timeout: 5000 });

    // Ensure we can select custom domain and different template
    await page.locator('text="Custom Domain"').click();
    await page.locator('text="Minimal"').click();

    // Click Support Agent explicitly to toggle
    const supportAgent = page.locator('text="Support Agent"');
    await expect(supportAgent).toBeVisible();
    await supportAgent.click();

    // Launch Store
    await page.locator('button:has-text("Launch Store")').click();

    // Verify completion
    await expect(page.locator('text="Building Your Business..."')).toBeVisible({ timeout: 5000 });

    await expect(page.locator('text="You\'re Live!"')).toBeVisible({ timeout: 30000 });
    await expect(page.locator('text="Your business has been successfully launched."')).toBeVisible();

    // Verify links are available
    const dashboardLink = page.locator('a:has-text("Go to Dashboard")');
    await expect(dashboardLink).toBeVisible();
    await expect(dashboardLink).toHaveAttribute('href', '/dashboard');
  });
});
