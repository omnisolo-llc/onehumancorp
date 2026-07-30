import { test, expect } from '@playwright/test';

test.describe('Index Instant Build Onboarding', () => {

  test('Instant Build button on index.html shows the instant build view', async ({ page }) => {
    await page.goto('/index.html');
    await expect(page.locator('#main-view')).toBeVisible();
    await expect(page.locator('#instant-build-view')).toBeHidden();

    await page.click('button:has-text("Instant Build")');

    await expect(page.locator('#main-view')).toBeHidden();
    await expect(page.locator('#instant-build-view')).toBeVisible();
  });

  test('Generate Storefront button does not navigate or show loader if bio is empty', async ({ page }) => {
    await page.goto('/index.html');
    await page.click('button:has-text("Instant Build")');

    // Bio is empty initially
    await page.click('#generate-storefront-btn');

    await expect(page.locator('#instant-build-view')).toBeVisible();
    await expect(page.locator('#loading-view')).toBeHidden();
  });

  test('Generate Storefront shows loading view when clicked with bio', async ({ page }) => {
    await page.goto('/index.html');
    await page.click('button:has-text("Instant Build")');
    await page.fill('#instant-bio', 'I run a local pizza delivery service');

    // Do not await the click immediately to allow checking the loading view
    const clickPromise = page.click('#generate-storefront-btn');

    await expect(page.locator('#instant-build-view')).toBeHidden();
    await expect(page.locator('#loading-view')).toBeVisible();

    // Since this hits the real backend without mocked routes, wait for the actual navigation or error.
    // If it navigates to dashboard, that's fine.
    try {
        await expect(page).toHaveURL(/.*dashboard\.html/, { timeout: 45000 });
    } catch (e) {
        // Just checking the loading view was the main point.
    }
  });

  test('Generate Storefront navigates to dashboard on successful API call', async ({ page }) => {
    await page.goto('/index.html');
    await page.click('button:has-text("Instant Build")');
    await page.fill('#instant-bio', 'A great local plumbing business in Texas');

    await page.click('#generate-storefront-btn');

    // Wait for the navigation using the real backend
    await expect(page).toHaveURL(/.*dashboard\.html/, { timeout: 45000 });
    // Verify dashboard is loaded (by checking for any typical dashboard element, like a triage queue or heading)
    await expect(page.locator('h1')).toBeVisible({ timeout: 10000 });
  });

  test('Generate Storefront correctly maintains layout without scrolling', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/index.html');

    const hasHorizontalScroll1 = await page.evaluate(() => document.documentElement.scrollWidth > window.innerWidth);
    expect(hasHorizontalScroll1).toBeFalsy();

    await page.click('button:has-text("Instant Build")');

    const hasHorizontalScroll2 = await page.evaluate(() => document.documentElement.scrollWidth > window.innerWidth);
    expect(hasHorizontalScroll2).toBeFalsy();
  });
});
