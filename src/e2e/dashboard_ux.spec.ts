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

  test('metrics cards should display side-by-side in a grid to conserve vertical space on mobile', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign In")');
    await page.waitForURL('**/*');

    const todaysSalesCard = page.locator('text=Today\'s Sales').locator('..').locator('..');
    const ordersToShipCard = page.locator('text=Orders to Ship').locator('..').locator('..');

    await expect(todaysSalesCard).toBeVisible();
    await expect(ordersToShipCard).toBeVisible();

    const todaysSalesBox = await todaysSalesCard.boundingBox();
    const ordersToShipBox = await ordersToShipCard.boundingBox();

    expect(todaysSalesBox).not.toBeNull();
    expect(ordersToShipBox).not.toBeNull();

    // Verify they are side-by-side by checking that their Y coordinates are the same
    expect(todaysSalesBox!.y).toBeCloseTo(ordersToShipBox!.y, 1);

    // Verify Today's Sales is to the left of Orders to Ship
    expect(todaysSalesBox!.x).toBeLessThan(ordersToShipBox!.x);
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
  await expect(page.locator('text=Grow My Business')).toBeVisible();
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

  test('tapping metrics cards should trigger the appropriate actions', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign In")');
    await page.waitForURL('**/*');

    // Click "Today's Sales" card
    const todaysSalesCard = page.locator('text=Today\'s Sales').locator('..').locator('..');
    await todaysSalesCard.click();
    // Assuming action_see_analytics() navigates to or opens Analytics
    await expect(page.locator('text=Analytics').first()).toBeVisible();

    // Navigate back to Dashboard if needed
    // (Assuming there's a back button or we just click Dashboard on nav)
    // For simplicity, we just reload or re-navigate to ensure clean state
    await page.goto('/');

    // Wait for Dashboard to load again
    await page.waitForURL('**/*');

    // Click "Orders to Ship" card
    const ordersToShipCard = page.locator('text=Orders to Ship').locator('..').locator('..');
    await ordersToShipCard.click();
    // Assuming action_view_orders() navigates to or opens Orders view
    await expect(page.locator('text=Orders').first()).toBeVisible();
  });
