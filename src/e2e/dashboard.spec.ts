import { test, expect } from '@playwright/test';

test.describe('Dashboard Core', () => {
  test('should load dashboard page', async ({ page }) => {
    await page.goto('/');
    await expect(page).toHaveTitle(/OneHuman/);
  });

  test('should display navigation', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('nav')).toBeVisible();
  });

  test('should show dashboard header', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Dashboard').first()).toBeVisible();
  });

  test('should display todays sales stat card', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Today\'s Sales')).toBeVisible();
  });

  test('should display new orders stat card', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Orders to Ship')).toBeVisible();
  });

  test('should display active AI helpers stat card', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Active Helpers')).toBeVisible();
  });

  test('should display tasks in progress stat card', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Active Help')).toBeVisible();
  });

  test('should display generative score stat card', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Generative Score')).toBeVisible();
  });

  test('should show quick actions section', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Business Advisory')).toBeVisible();
  });

  test('should display add product quick action', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Add Product')).toBeVisible();
  });

  test('should display view orders quick action', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=View Orders')).toBeVisible();
  });

  test('should display messages quick action', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Messages')).toBeVisible();
  });

  test('should display analytics quick action', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Analytics')).toBeVisible();
  });

  test('should display share store quick action', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Share Store')).toBeVisible();
  });

  test('should show drafts ready for review section', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Drafts Ready for Review')).toBeVisible();
  });


  test('should show my team section', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=My Team')).toBeVisible();
  });

  test('should display help buttons', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Ask AI')).toBeVisible();
    await expect(page.locator('text=Help')).toBeVisible();
  });

  test('should display billing button', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Billing')).toBeVisible();
  });

  test('should display docs button', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Docs')).toBeVisible();
  });

  test('should display videos button', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Videos')).toBeVisible();
  });

  test('should display tour button', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Tour')).toBeVisible();
  });

  test('should display whats new button', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=What\'s New')).toBeVisible();
  });

  test('should navigate to dashboard from home', async ({ page }) => {
    await page.goto('/');
    await page.click('text=Dashboard');
    await expect(page).toHaveURL(/\//);
  });

  test('should show draft approval buttons', async ({ page }) => {
    await page.goto('/');
    const approveBtn = page.locator('button:has-text("Approve")').first();
    const editBtn = page.locator('button:has-text("Edit")').first();
    await expect(approveBtn).toBeVisible();
    await expect(editBtn).toBeVisible();
  });

});

test.describe('Dashboard Mobile', () => {
  test.use({ viewport: { width: 375, height: 800 } });

  test('should display correctly on mobile', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Dashboard').first()).toBeVisible();
  });

  test('should verify plain language labels on mobile', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Today\'s Sales')).toBeVisible();
    await expect(page.locator('text=Orders to Ship')).toBeVisible();
    await expect(page.locator('text=Generative Score')).toBeVisible();
    await expect(page.locator('text=Drafts Ready for Review')).toBeVisible();
  });

  test('should display quick actions on mobile', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('text=Business Advisory')).toBeVisible();
  });
});