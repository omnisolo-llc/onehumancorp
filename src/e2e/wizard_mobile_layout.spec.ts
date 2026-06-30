import { test, expect } from './fixtures';

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

    await expect(page.locator('text="Tell us about your business"').first()).toBeVisible();

    // Check click routing inside builder
    await page.locator('text="Step-by-Step Setup"').click();

    await expect(page.getByRole('heading', { name: 'How do you work?' })).toBeVisible();
    await page.getByText("I'm a Baker").click();
    await page.locator('#step-context .next-step-btn').click();

    await expect(page.locator('#business-categories')).toBeVisible();
    await page.locator('#business-categories').selectOption('Bakery');
    await page.locator('#step-categories .next-step-btn').click();

    await expect(page.getByRole('heading', { name: /What's the name of your business\?/ })).toBeVisible();
  });

  test('Main Onboarding multi-step wizard mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/setup.html');

    await expect(page.locator('text="Tell us about your business"').first()).toBeVisible();
    await page.locator('text="Step-by-Step Setup"').click();

    await expect(page.getByRole('heading', { name: 'How do you work?' })).toBeVisible();

    // Check constraints are working inside inputs.
    await page.getByText("I'm a Baker").click();
    await page.locator('#step-context .next-step-btn').click();
    await expect(page.locator('#business-categories')).toBeVisible();
    await page.locator('#business-categories').selectOption('Bakery');
    await page.locator('#step-categories .next-step-btn').click();

    await expect(page.getByRole('heading', { name: /What's the name of your business\?/ })).toBeVisible();
    await page.getByPlaceholder('e.g. Maya\'s Custom Cakes').fill('Cakes By Maya');
    await page.locator('#step-name .next-step-btn').click();

    await expect(page.getByRole('heading', { name: 'Set up your Assistant' })).toBeVisible();
  });

  test('Direct routing for business-setup compatibility page', async ({ page }) => {
    await page.goto('/business-setup');

    // Should immediately reroute to onboarding
    await expect(page.locator('text="Tell us about your business"').first()).toBeVisible();
  });

  test('Onboarding allows full traversal on standard layout', async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 900 });
    await page.goto('/setup.html');

    await expect(page.locator('text="Tell us about your business"').first()).toBeVisible();
    await page.locator('text="Step-by-Step Setup"').click();

    await expect(page.getByRole('heading', { name: 'How do you work?' })).toBeVisible();
    await page.getByText("I'm a Baker").click();
    await page.locator('#step-context .next-step-btn').click();

    await expect(page.locator('#business-categories')).toBeVisible();
  });

  test('Loading state padding check on mobile layout', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/onboarding');

    // Attempt to access step 4 loading state directly if possible, or intercept network and check
    await page.evaluate(() => {
        window.localStorage.setItem('onboarding-storage-v4', JSON.stringify({
            state: { step: 4 }
        }));
    });

    await page.reload();

    // Check loading indicator container doesn't overflow
    const container = page.locator('.animate-fade-in');
    await expect(container).toBeVisible();

    const containerWidth = await container.evaluate(el => el.clientWidth);
    expect(containerWidth).toBeLessThanOrEqual(375);
  });
});
