import { test, expect } from '@playwright/test';

test.describe('Onboarding Error Banner UI', () => {

  test.beforeEach(async ({ page }) => {
    // Navigate to the onboarding flow where the error banner would be tested
    await page.goto('/onboarding');
    await page.addInitScript(() => {
      window.localStorage.clear();
    });
  });

  test('Error banner should render correctly with premium macOS translucent aesthetics when API fails', async ({ page }) => {
    await page.route('**/api/onboarding/start', route => {
      route.abort('failed');
    });

    // Proceed to the end of the form by clicking through steps
    await expect(page.getByText("Setup Assistant")).toBeVisible();
    await page.getByRole('button', { name: 'Start My Business' }).click();
    await expect(page.getByText("What's the name of your business?")).toBeVisible();

    await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Test Business');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Testing business setup.');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder(/Portland, OR/i).fill('Seattle, WA');
    await page.getByRole('button', { name: 'Next' }).click();

    await page.getByPlaceholder(/Local families, Tech startups/i).fill('Everyone');
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByText("Review Details")).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: 'Continue' }).click();

    await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Admin User');
    await page.getByPlaceholder(/you@example.com/i).fill('admin@example.com');
    await page.getByPlaceholder(/••••••••/i).fill('password123');

    // Click the final "Approve & Publish" submit button
    await page.getByRole('button', { name: 'Approve & Publish' }).click();

    // The error banner should now be visible
    const errorBanner = page.getByText(/Failed to fetch|Backend connection failed/i);
    await expect(errorBanner).toBeVisible();

    // Check that it has the premium glassmorphism styling
    const bannerContainer = errorBanner.locator('..');
    await expect(bannerContainer).toHaveClass(/backdrop-blur/);
    await expect(bannerContainer).toHaveClass(/border-\[\#FF3B30\]\/50/);
    await expect(bannerContainer).toHaveClass(/text-\[\#FF3B30\]/);
    await expect(bannerContainer).toHaveClass(/animate-shake/);
  });

  test('Error banner should remain pinned outside scroll area when scrolling down the form', async ({ page }) => {
    // Setup error state
    await page.route('**/api/onboarding/start', route => {
      route.abort('failed');
    });

    await page.getByRole('button', { name: 'Start My Business' }).click();
    await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Test');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Test');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.getByPlaceholder(/Portland, OR/i).fill('Test');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.getByPlaceholder(/Local families, Tech startups/i).fill('Test');
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByText("Review Details")).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: 'Continue' }).click();

    await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Admin User');
    await page.getByPlaceholder(/you@example.com/i).fill('admin@example.com');
    await page.getByPlaceholder(/••••••••/i).fill('password123');
    await page.getByRole('button', { name: 'Approve & Publish' }).click();

    // Verify error banner is visible
    const errorBanner = page.getByText(/Failed to fetch|Backend connection failed/i);
    await expect(errorBanner).toBeVisible();

    // Check that the error banner is NOT inside the custom-scrollbar container
    const scrollContainer = page.locator('.custom-scrollbar');
    // We expect 0 count of error banners INSIDE the scroll container, it should be above it
    await expect(scrollContainer.getByText(/Failed to fetch|Backend connection failed/i)).toHaveCount(0);
  });

  test('Error banner should disappear if a subsequent submission succeeds', async ({ page }) => {
    // First, mock the network request to fail
    await page.route('**/api/onboarding/start', route => {
      route.abort('failed');
    }, { times: 1 });

    await page.getByRole('button', { name: 'Start My Business' }).click();
    await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Test Business');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Testing business setup.');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.getByPlaceholder(/Portland, OR/i).fill('Seattle, WA');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.getByPlaceholder(/Local families, Tech startups/i).fill('Everyone');
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByText("Review Details")).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: 'Continue' }).click();

    await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Admin User');
    await page.getByPlaceholder(/you@example.com/i).fill('admin@example.com');
    await page.getByPlaceholder(/••••••••/i).fill('password123');

    // Click the final "Approve & Publish" submit button
    await page.getByRole('button', { name: 'Approve & Publish' }).click();

    // The error banner should now be visible
    const errorBanner = page.getByText(/Failed to fetch|Backend connection failed/i);
    await expect(errorBanner).toBeVisible();

    // Now mock the request to succeed
    await page.unroute('**/api/onboarding/start');

    // Resubmit
    await page.getByRole('button', { name: 'Approve & Publish' }).click();

    // The error banner should disappear
    await expect(errorBanner).not.toBeVisible();
    await expect(page.getByText("Building Your Business...")).toBeVisible();
  });

  test('Error banner renders accurately on small mobile screens without horizontal overflow', async ({ page }) => {
    // Set mobile viewport size
    await page.setViewportSize({ width: 375, height: 667 });

    // Mock failure
    await page.route('**/api/onboarding/start', route => {
      route.abort('failed');
    });

    await page.getByRole('button', { name: 'Start My Business' }).click();
    await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Test');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Test');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.getByPlaceholder(/Portland, OR/i).fill('Test');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.getByPlaceholder(/Local families, Tech startups/i).fill('Test');
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByText("Review Details")).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: 'Continue' }).click();

    await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Admin User');
    await page.getByPlaceholder(/you@example.com/i).fill('admin@example.com');
    await page.getByPlaceholder(/••••••••/i).fill('password123');
    await page.getByRole('button', { name: 'Approve & Publish' }).click();

    // Verify error banner is visible
    const errorBanner = page.getByText(/Failed to fetch|Backend connection failed/i);
    await expect(errorBanner).toBeVisible();

    // Verify no horizontal scrolling on the page body
    const bodyBox = await page.locator('body').boundingBox();
    expect(bodyBox?.width).toBeLessThanOrEqual(375);
  });

  test('Error banner shows the alert icon alongside the text', async ({ page }) => {
    // Mock failure
    await page.route('**/api/onboarding/start', route => {
      route.abort('failed');
    });

    await page.getByRole('button', { name: 'Start My Business' }).click();
    await page.getByPlaceholder(/Maya's Custom Cake/i).fill('Test');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Test');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.getByPlaceholder(/Portland, OR/i).fill('Test');
    await page.getByRole('button', { name: 'Next' }).click();
    await page.getByPlaceholder(/Local families, Tech startups/i).fill('Test');
    await page.getByRole('button', { name: 'Next' }).click();

    await expect(page.getByText("Review Details")).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: 'Continue' }).click();

    await page.getByPlaceholder(/e.g. Maya Smith/i).fill('Admin User');
    await page.getByPlaceholder(/you@example.com/i).fill('admin@example.com');
    await page.getByPlaceholder(/••••••••/i).fill('password123');
    await page.getByRole('button', { name: 'Approve & Publish' }).click();

    // Verify error banner and icon
    const errorBannerContainer = page.getByText(/Failed to fetch|Backend connection failed/i).locator("..");
    await expect(errorBannerContainer.locator('svg')).toBeVisible();
  });
});
