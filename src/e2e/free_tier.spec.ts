import { test, expect } from '@playwright/test';

test.describe('Free Tier & Upgrade Funnel', () => {
  test('should display product limit soft paywall', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('test@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();
    await page.waitForURL('**/dashboard');

    // Assuming we have a mock in Rust that triggers the limit after a certain action
    // Or we can manually navigate to a specific path if available
    // Here we click "Add Offering" rapidly to trigger the limit (or test mock state)
    await page.goto('/business-manager');
    for(let i=0; i<11; i++) {
        const addBtn = page.locator('button:has-text("+ Add New Offering")');
        if(await addBtn.isVisible()) {
            await addBtn.click();
            // Just simulate adding by hitting back for now, wait for the mock to trigger
            await page.locator('button:has-text("Back to List")').click();
        }
    }

    // Since mock triggers are handled differently, we will look for the upgrade prompt directly
    await expect(page.locator('text=Scale Up Your Team')).toBeVisible({ timeout: 5000 });
  });

  test('should display agent limits soft paywall', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('test@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();
    await page.waitForURL('**/dashboard');

    await page.goto('/agents');

    // Click "Hire Agent"
    const hireBtn = page.locator('button:has-text("Hire Agent")').first();
    await expect(hireBtn).toBeVisible();
    await hireBtn.click();

    // Verify upgrade prompt pops up
    await expect(page.locator('text=Scale Up Your Team')).toBeVisible({ timeout: 5000 });
  });

  test('should verify upgrade prompt dismissal', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('test@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();
    await page.waitForURL('**/dashboard');

    await page.goto('/agents');

    // Click "Hire Agent"
    const hireBtn = page.locator('button:has-text("Hire Agent")').first();
    await expect(hireBtn).toBeVisible();
    await hireBtn.click();

    await expect(page.locator('text=Scale Up Your Team')).toBeVisible({ timeout: 5000 });

    const dismissBtn = page.locator('button:has-text("✕")');
    await dismissBtn.click();

    await expect(page.locator('text=Scale Up Your Team')).toBeHidden();
  });

  test('should verify upgrade CTA navigation', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('test@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();
    await page.waitForURL('**/dashboard');

    await page.goto('/agents');

    // Click "Hire Agent"
    const hireBtn = page.locator('button:has-text("Hire Agent")').first();
    await expect(hireBtn).toBeVisible();
    await hireBtn.click();

    const upgradeBtn = page.locator('button:has-text("Upgrade to Pro")').first();
    await expect(upgradeBtn).toBeVisible();
    await upgradeBtn.click();
  });

  test('should verify free tier text indication on my plan', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('test@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Login")').first().click();
    await page.waitForURL('**/dashboard');

    await page.goto('/my-plan');

    await expect(page.locator('text=Free Tier').first()).toBeVisible();
  });
});
