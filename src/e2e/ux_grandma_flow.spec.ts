import { test, expect } from '@playwright/test';

test.describe('Grandmother UX End-to-End Flow Validation', () => {
  test.use({ viewport: { width: 375, height: 800 } });

  test('Flow 1: First-time user logs in and sees plain language headers', async ({ page }) => {
    await page.goto('/login');
    // Verify login screen uses the friendly start button text
    await expect(page.locator('button:has-text("🚀 Start Business Setup")')).toBeVisible();

    await page.fill('input[type="email"]', 'grandma@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign In")');
    await page.waitForURL('**/*');

    await expect(page.locator('text=My Business').first()).toBeVisible();
    await expect(page.locator('text=Quick Actions')).toBeVisible();
  });

  test('Flow 2: User opens Quick Actions helper for guidance', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'grandma@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign In")');
    await page.waitForURL('**/*');

    const questionMarkBtn = page.locator('text="Quick Actions"').locator('..').locator('button:has-text("?")');
    await expect(questionMarkBtn).toBeVisible();
    await questionMarkBtn.click();

    // Verify the new plain language hint is displayed
    await expect(page.locator('text=These buttons are shortcuts to your most common daily tasks.')).toBeVisible();
  });

  test('Flow 3: User accesses Menu and sees simple connection labels', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'grandma@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign In")');
    await page.waitForURL('**/*');

    const menuBtn = page.locator('button:has-text("Menu")');
    await expect(menuBtn).toBeVisible();
    await menuBtn.click();

    // Verify straightforward options in the menu
    await expect(page.locator('button:has-text("Connect Apps")')).toBeVisible();
    await expect(page.locator('button:has-text("Video Tutorials")')).toBeVisible();
  });

  test('Flow 4: User navigates to Connect Apps to review available connections', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'grandma@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign In")');
    await page.waitForURL('**/*');

    await page.click('button:has-text("Menu")');
    await page.click('button:has-text("Connect Apps")');

    // Verify API screen uses grandma-friendly terms
    await expect(page.locator('text=Custom Integration')).toBeVisible();
  });

  test('Flow 5: User initiates guided setup process from login screen', async ({ page }) => {
    await page.goto('/login');
    const startBusinessBtn = page.locator('button:has-text("🚀 Start Business Setup")');
    await expect(startBusinessBtn).toBeVisible();
    await startBusinessBtn.click();

    // The setup wizard should appear
    // We expect the first setup wizard text / step to be visible
    await expect(page.locator('text="Welcome"').first()).toBeVisible({ timeout: 5000 });
  });
});
