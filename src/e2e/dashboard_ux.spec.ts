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
    await expect(page.locator('text=Active Help')).toBeVisible();

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
  await expect(page.locator('text=Business Advisory')).toBeVisible();

  // Verify First-Time User Tour ? icon toggle
  const questionMarkBtn = page.locator('button:has-text("?")');
  await expect(questionMarkBtn).toBeVisible();

  // Verify tap targets are appropriately sized (>= 44px)
  const box = await questionMarkBtn.boundingBox();
  expect(box?.height).toBeGreaterThanOrEqual(44);
  expect(box?.width).toBeGreaterThanOrEqual(44);

  await questionMarkBtn.click();
  await expect(page.locator('text=These buttons are shortcuts to your most common daily tasks.')).toBeVisible();

  // Verify bottom navigation bar buttons are present
  const btnAdd = page.locator('button:has-text("Add")');
  await expect(btnAdd).toBeVisible();
  const boxAdd = await btnAdd.boundingBox();
  expect(boxAdd?.height).toBeGreaterThanOrEqual(44);
  expect(boxAdd?.width).toBeGreaterThanOrEqual(44);

  const btnOrders = page.locator('button:has-text("Orders")');
  await expect(btnOrders).toBeVisible();
  const boxOrders = await btnOrders.boundingBox();
  expect(boxOrders?.height).toBeGreaterThanOrEqual(44);
  expect(boxOrders?.width).toBeGreaterThanOrEqual(44);

  const btnChat = page.locator('button:has-text("Chat")');
  await expect(btnChat).toBeVisible();
  const boxChat = await btnChat.boundingBox();
  expect(boxChat?.height).toBeGreaterThanOrEqual(44);
  expect(boxChat?.width).toBeGreaterThanOrEqual(44);

  const btnStats = page.locator('button:has-text("Stats")');
  await expect(btnStats).toBeVisible();
  const boxStats = await btnStats.boundingBox();
  expect(boxStats?.height).toBeGreaterThanOrEqual(44);
  expect(boxStats?.width).toBeGreaterThanOrEqual(44);

  const btnShare = page.locator('button:has-text("Share")');
  await expect(btnShare).toBeVisible();
  const boxShare = await btnShare.boundingBox();
  expect(boxShare?.height).toBeGreaterThanOrEqual(44);
  expect(boxShare?.width).toBeGreaterThanOrEqual(44);
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
  await expect(page.locator('button:has-text("Connect Apps")')).toBeVisible();
  await expect(page.locator('button:has-text("Video Tutorials")')).toBeVisible();
  await expect(page.locator('button:has-text("App Tour")')).toBeVisible();
  await expect(page.locator('button:has-text("What\'s New")')).toBeVisible();
});
