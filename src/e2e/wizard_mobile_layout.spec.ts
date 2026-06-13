import { test, expect } from '@playwright/test';

test.describe('Wizard and Onboarding flows', () => {

  test('Website builder wizard mobile layout', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });

    await page.goto('/website-builder');

    // Check elements
    const heading = page.getByRole('heading', { name: '10-Minute Setup Wizard' });
    await expect(heading).toBeVisible();

    // Verify it doesn't overflow horizontally
    const htmlWidth = await page.evaluate(() => document.documentElement.scrollWidth);
    const windowWidth = await page.evaluate(() => window.innerWidth);
    expect(htmlWidth).toBeLessThanOrEqual(windowWidth);

    await page.getByRole('button', { name: 'Instant Build' }).click();
    await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();
  });

  test('Builder mobile UI test', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/builder');

    await expect(page.getByText('What are you building today?')).toBeVisible();

    // Check click routing inside builder
    await page.getByText('Selling Products').click();
    await expect(page.getByText("Let's build your store")).toBeVisible();

    const nameInput = page.getByPlaceholder('e.g. Acme Corp');
    await expect(nameInput).toBeVisible();
    await nameInput.fill('Maya Cakes');

    const descInput = page.getByPlaceholder('e.g. Retail, Consulting, Tech');
    await expect(descInput).toBeVisible();
    await descInput.fill('Bakery');

    await page.getByRole('button', { name: 'Next: Choose Vibe' }).click();
    await expect(page.getByText('Select Your Vibe')).toBeVisible();
  });

  test('Main Onboarding multi-step wizard mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/onboarding');

    await expect(page.getByText('What are you building today?')).toBeVisible();
    await page.getByText('Selling Products').click();

    await expect(page.getByText('Business Name')).toBeVisible();

    // Check constraints are working inside inputs.
    await page.getByPlaceholder('e.g. Acme Corp').fill('Cakes By Maya');
    await page.getByPlaceholder('e.g. Retail, Consulting, Tech').fill('Baker');

    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByText('Select Your Vibe')).toBeVisible();
    await page.getByText('Friendly').click();
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByText('Final Details')).toBeVisible();
  });

  test('Direct routing for business-setup compatibility page', async ({ page }) => {
    await page.goto('/business-setup');

    // Should immediately reroute to onboarding
    await expect(page.getByText('What are you building today?')).toBeVisible();
  });

  test('Onboarding allows full traversal on standard layout', async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 900 });
    await page.goto('/onboarding');

    await expect(page.getByText('What are you building today?')).toBeVisible();
    await page.getByText('Offering Services').click();

    await expect(page.getByText('Business Name')).toBeVisible();

    await page.getByPlaceholder('e.g. Acme Corp').fill('Auto Repair');
    await page.getByPlaceholder('e.g. Retail, Consulting, Tech').fill('Mechanic');

    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByText('Select Your Vibe')).toBeVisible();
  });
});
