import { test, expect } from './fixtures';

test.describe('Invisible Storefront Generator Flow', () => {
  test('uses the instant build text area to autonomously generate a storefront', async ({ page }) => {
    const id = `invisible-gen-${Date.now()}-${Math.random()}`;
    await page.addInitScript((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('user_id', tenantId);
      localStorage.removeItem('ohc_builder_status');
    }, id);

    await page.goto('/website-builder');

    // Check if we are on the welcome step
    await expect(page.getByRole('heading', { name: 'Your business, live in minutes.' })).toBeVisible();

    // Click Instant Build button to go to 'instant-build' step
    await page.getByRole('button', { name: /Instant Build/ }).click();

    // Verify the step changed
    await expect(page.getByRole('heading', { name: 'Describe your business in a sentence' })).toBeVisible();

    // Fill in the description
    const description = "I run a vegan bakery in Portland called Maya's Vegan Bakery";
    await page.getByPlaceholder('e.g. I run a local bakery').fill(description);

    // Click Generate Storefront
    await page.getByRole('button', { name: /Generate Storefront/ }).click();

    // Expect loading state
    await expect(page.getByText('Agents are building your store...')).toBeVisible();
    await expect(page.getByText('Generating...')).toBeVisible(); // from the button if it's visible, but the whole page changes to a loader. Wait, the page changes to a loader.

    // Expect success state
    await expect(page.getByRole('heading', { name: /Success! Your business is live!/ })).toBeVisible({ timeout: 15000 });

    // Expect the copy URL section to be visible
    await expect(page.getByText("You're set up! Here's what to do next:")).toBeVisible();
    await expect(page.getByRole('button', { name: /View Welcome Checklist/ })).toBeVisible();
  });

  test('handles errors from the intake API gracefully', async ({ page }) => {
    await page.route('**/api/onboarding/intake', route => route.fulfill({
      status: 500,
      body: JSON.stringify({ error: 'Internal Server Error' })
    }));

    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Instant Build/ }).click();
    await page.getByPlaceholder('e.g. I run a local bakery').fill('Failing test case');
    await page.getByRole('button', { name: /Generate Storefront/ }).click();

    await expect(page.getByText('Failed to process business details')).toBeVisible();
  });

  test('handles errors from the start API gracefully', async ({ page }) => {
    await page.route('**/api/onboarding/intake', route => route.fulfill({
      status: 200,
      body: JSON.stringify({ business_name: 'Test', business_type: 'Store', categories: [], initial_products: [] })
    }));
    await page.route('**/api/onboarding/start', route => route.fulfill({
      status: 500,
      body: JSON.stringify({ error: 'Internal Server Error' })
    }));

    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Instant Build/ }).click();
    await page.getByPlaceholder('e.g. I run a local bakery').fill('Failing test case');
    await page.getByRole('button', { name: /Generate Storefront/ }).click();

    await expect(page.getByText('Failed to start onboarding')).toBeVisible();
  });

  test('button is disabled while generating', async ({ page }) => {
    // delay the route to check state
    await page.route('**/api/onboarding/intake', async route => {
      await new Promise(r => setTimeout(r, 1000));
      route.fulfill({
        status: 200,
        body: JSON.stringify({ business_name: 'Test', business_type: 'Store', categories: [], initial_products: [] })
      });
    });

    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Instant Build/ }).click();
    await page.getByPlaceholder('e.g. I run a local bakery').fill('Test case');
    const genButton = page.getByRole('button', { name: /Generate Storefront/ });
    await genButton.click();

    // Check loading button
    await expect(genButton).toBeDisabled();
    await expect(page.getByText('Generating...')).toBeVisible();
  });

  test('button is disabled when input is empty', async ({ page }) => {
    await page.goto('/website-builder');
    await page.getByRole('button', { name: /Instant Build/ }).click();
    const genButton = page.getByRole('button', { name: /Generate Storefront/ });
    await expect(genButton).toBeDisabled();

    await page.getByPlaceholder('e.g. I run a local bakery').fill(' ');
    await expect(genButton).toBeDisabled();

    await page.getByPlaceholder('e.g. I run a local bakery').fill('Test');
    await expect(genButton).toBeEnabled();
  });

});
