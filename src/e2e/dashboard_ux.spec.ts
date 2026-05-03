import { test, expect } from '@playwright/test';

test.describe('Dashboard UX', () => {
  test.use({ viewport: { width: 375, height: 800 } });

  test('should display correctly on mobile and verify plain language labels', async ({ page }) => {
    // Navigate to login page
    await page.goto('/login');

    // Fill in credentials and sign in
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign In")');

    // Wait for Dashboard to load
    await page.waitForURL('**/*');

    // Some apps navigate to '/' or '/dashboard', we will just wait for navigation
    // and verify the labels.
    await expect(page.locator('text=My Business').first()).toBeVisible();
    await expect(page.locator('text=Today\'s Sales')).toBeVisible();
    await expect(page.locator('text=Orders to Ship')).toBeVisible();
    await expect(page.locator('text=Active Helpers')).toBeVisible();
    await expect(page.locator('text=Current AI Tasks')).toBeVisible();

    // Verify softer wording for drafts
    await expect(page.locator('text=Drafts Ready for Review')).toBeVisible();
  });
});

test('should display Quick Actions on mobile', async ({ page }) => {
  await page.goto('/login');
  await page.fill('input[type="email"]', 'test@example.com');
  await page.fill('input[type="password"]', 'password123');
  await page.click('button:has-text("Sign In")');
  await page.waitForURL('**/*');

  // Verify navigation actions
  await expect(page.locator('text=Quick Actions')).toBeVisible();

  // Verify First-Time User Tour ? icon toggle
  const questionMarkBtn = page.locator('button:has-text("?")');
  await expect(questionMarkBtn).toBeVisible();

  // Verify tap targets are appropriately sized (>= 44px)
  const box = await questionMarkBtn.boundingBox();
  expect(box?.height).toBeGreaterThanOrEqual(44);
  expect(box?.width).toBeGreaterThanOrEqual(44);

  await questionMarkBtn.click();
  await expect(page.locator('text=These buttons are shortcuts to your most common daily tasks.')).toBeVisible();

  await expect(page.locator('text=Add Product')).toBeVisible();
  await expect(page.locator('text=View Orders')).toBeVisible();
  await expect(page.locator('text=Messages')).toBeVisible();
  await expect(page.locator('text=Analytics')).toBeVisible();
  await expect(page.locator('text=Share Store')).toBeVisible();
});

test('should display Menu toggle on mobile and have expected links', async ({ page }) => {
  await page.goto('/login');
  await page.fill('input[type="email"]', 'test@example.com');
  await page.fill('input[type="password"]', 'password123');
  await page.click('button:has-text("Sign In")');
  await page.waitForURL('**/*');

  // Verify navigation actions
  const menuBtn = page.locator('button:has-text("Menu")');
  await expect(menuBtn).toBeVisible();
  await menuBtn.click();

  await expect(page.locator('button:has-text("Help Center")')).toBeVisible();
  await expect(page.locator('button:has-text("Billing")')).toBeVisible();
  await expect(page.locator('button:has-text("API Docs")')).toBeVisible();
  await expect(page.locator('button:has-text("Video Tutorials")')).toBeVisible();
  await expect(page.locator('button:has-text("App Tour")')).toBeVisible();
  await expect(page.locator('button:has-text("What\'s New")')).toBeVisible();
});

test('dashboard stats layout is responsive and visible without horizontal scroll', async ({ page }) => {
  // Mobile test width 375px
  await page.setViewportSize({ width: 375, height: 800 });

  // Starting from the home page as required by the e2e standard.
  await page.goto('/login');

  await page.fill('input[type="email"]', 'test@example.com');
  await page.fill('input[type="password"]', 'password123');
  await page.click('button:has-text("Sign In")');

  await page.waitForURL('**/*');

  // Wait for dashboard title to be present.
  await expect(page.locator('text=My Business').first()).toBeVisible({ timeout: 15000 });

  // Validate no horizontal scroll overflow
  const isScrollable = await page.evaluate(() => {
    return document.documentElement.scrollWidth > document.documentElement.clientWidth;
  });

  expect(isScrollable).toBe(false);
});
