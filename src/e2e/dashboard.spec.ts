import { test, expect } from '@playwright/test';

test.describe('Dashboard Core', () => {
  test.beforeEach(async ({ page }) => {
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'admin123');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page).toHaveURL(/\/(dashboard|\/)$/, { timeout: 10000 });

    // Navigate to feature from dashboard
    const btn = page.locator('button:has-text("/login")').first();
    if (await btn.isVisible()) {
      await btn.click();
    } else {
      const menuBtn = page.locator('button:has-text("Menu")').first();
      if (await menuBtn.isVisible()) {
        await menuBtn.click();
        const innerBtn = page.locator('button:has-text("/login")').first();
        if (await innerBtn.isVisible()) {
          await innerBtn.click();
        }
      }
    }
  });

  test.beforeEach(async ({ page }) => {
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'admin123');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page).toHaveURL(/\/(dashboard|\/)$/, { timeout: 10000 });

    const btn = page.locator('button:has-text("/login")');
    if (await btn.isVisible()) {
      await btn.click();
    } else {
      const menuBtn = page.locator('button:has-text("Menu")');
      if (await menuBtn.isVisible()) {
        await menuBtn.click();
        await page.locator('button:has-text("/login")').click();
      }
    }
  });
  test('should load dashboard page', async ({ page }) => {
    await expect(page).toHaveTitle(/OneHuman/);
  });

  test('should display navigation', async ({ page }) => {
    await expect(page.locator('nav')).toBeVisible();
  });

  test('should show dashboard header', async ({ page }) => {
    await expect(page.locator('text=Dashboard').first()).toBeVisible();
  });

  test('should display todays sales stat card', async ({ page }) => {
    await expect(page.locator('text=Today\'s Sales')).toBeVisible();
  });

  test('should display new orders stat card', async ({ page }) => {
    await expect(page.locator('text=New Orders')).toBeVisible();
  });

  test('should display active AI helpers stat card', async ({ page }) => {
    await expect(page.locator('text=Active AI Helpers')).toBeVisible();
  });

  test('should display tasks in progress stat card', async ({ page }) => {
    await expect(page.locator('text=Tasks in Progress')).toBeVisible();
  });

  test('should show quick actions section', async ({ page }) => {
    await expect(page.locator('text=Quick Actions')).toBeVisible();
  });

  test('should display add product quick action', async ({ page }) => {
    await expect(page.locator('text=Add Product')).toBeVisible();
  });

  test('should display view orders quick action', async ({ page }) => {
    await expect(page.locator('text=View Orders')).toBeVisible();
  });

  test('should display messages quick action', async ({ page }) => {
    await expect(page.locator('text=Messages')).toBeVisible();
  });

  test('should display analytics quick action', async ({ page }) => {
    await expect(page.locator('text=Analytics')).toBeVisible();
  });

  test('should display share store quick action', async ({ page }) => {
    await expect(page.locator('text=Share Store')).toBeVisible();
  });

  test('should show drafts ready for review section', async ({ page }) => {
    await expect(page.locator('text=Drafts Ready for Review')).toBeVisible();
  });

  test('should display swarm observability section', async ({ page }) => {
    await expect(page.locator('text=Swarm Observability')).toBeVisible();
  });

  test('should show company structure section', async ({ page }) => {
    await expect(page.locator('text=Company Structure')).toBeVisible();
  });

  test('should display aligned company structure list', async ({ page }) => {
    const btn1 = page.locator('button:has-text("-")').first();
    await expect(btn1).toBeVisible();
  });

  test('should display help buttons', async ({ page }) => {
    await expect(page.locator('text=Ask AI')).toBeVisible();
    await expect(page.locator('text=Help')).toBeVisible();
  });

  test('should display billing button', async ({ page }) => {
    await expect(page.locator('text=Billing')).toBeVisible();
  });

  test('should display docs button', async ({ page }) => {
    await expect(page.locator('text=Docs')).toBeVisible();
  });

  test('should display videos button', async ({ page }) => {
    await expect(page.locator('text=Videos')).toBeVisible();
  });

  test('should display tour button', async ({ page }) => {
    await expect(page.locator('text=Tour')).toBeVisible();
  });

  test('should display whats new button', async ({ page }) => {
    await expect(page.locator('text=What\'s New')).toBeVisible();
  });

  test('should navigate to dashboard from home', async ({ page }) => {
    await page.click('text=Dashboard');
    await expect(page).toHaveURL(/\//);
  });

  test('should show draft approval buttons', async ({ page }) => {
    const approveBtn = page.locator('button:has-text("Approve")').first();
    const editBtn = page.locator('button:has-text("Edit")').first();
    await expect(approveBtn).toBeVisible();
    await expect(editBtn).toBeVisible();
  });

  test('should display agent activity feed', async ({ page }) => {
    const activitySection = page.locator('[class*="activity"], [class*="feed"]').first();
    await expect(activitySection).toBeVisible();
  });
});

test.describe('Dashboard Mobile', () => {
  test.beforeEach(async ({ page }) => {
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'admin123');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page).toHaveURL(/\/(dashboard|\/)$/, { timeout: 10000 });

    // Navigate to feature from dashboard
    const btn = page.locator('button:has-text("/login")').first();
    if (await btn.isVisible()) {
      await btn.click();
    } else {
      const menuBtn = page.locator('button:has-text("Menu")').first();
      if (await menuBtn.isVisible()) {
        await menuBtn.click();
        const innerBtn = page.locator('button:has-text("/login")').first();
        if (await innerBtn.isVisible()) {
          await innerBtn.click();
        }
      }
    }
  });

  test.use({ viewport: { width: 375, height: 800 } });

  test('should display correctly on mobile', async ({ page }) => {
    await expect(page.locator('text=Dashboard').first()).toBeVisible();
  });

  test('should verify plain language labels on mobile', async ({ page }) => {
    await expect(page.locator('text=Today\'s Sales')).toBeVisible();
    await expect(page.locator('text=New Orders')).toBeVisible();
    await expect(page.locator('text=Drafts Ready for Review')).toBeVisible();
  });

  test('should display quick actions on mobile', async ({ page }) => {
    await expect(page.locator('text=Quick Actions')).toBeVisible();
  });
});
