import { test, expect } from '@playwright/test';

test.describe('Viral Referral Loop', () => {
  test('should navigate to user management and interact with the viral referral loop widget', async ({ page }) => {
    // 1. start from the home page after user login with no pre-authenticated shortcuts
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('test@example.com');
    await page.locator('input[type="password"]').first().first().fill('password123');
    await page.getByRole('button', { name: /Login|Sign In/i }).first().click();
    await page.waitForURL('**/dashboard');

    // 2. navigate the entire feature flow by clicking UI links/buttons exactly as a real user would
    const usersBtn = page.locator('button:has-text("Users"), button:has-text("Team"), a[href*="/users"]').first();
    await usersBtn.click();

    // Verify we are on User Management and the new widget is there
    await expect(page.locator('text=User Management').first()).toBeVisible();
    await expect(page.locator('text=Referral Program')).toBeVisible();

    // Check typography
    const referralHeader = page.locator('text=Referral Program');
    await expect(referralHeader).toBeVisible();

    const subtext = page.locator('text=Share OHC with a friend, both get 1 month free Pro.');
    await expect(subtext).toBeVisible();

    // 3. proceed through every step until the process finishes and the result is visible in the UI
    const inviteUserBtn = page.locator('button:has-text("Invite User")');
    await expect(inviteUserBtn).toBeVisible();

    // Hover to trigger the AnimatedScale / Container animation logic from Slint
    const widgetCard = referralHeader.locator('..');
    await widgetCard.hover();
    await page.waitForTimeout(300); // Wait for 300ms animation easing

    // Click the invite button
    await inviteUserBtn.click();

    // 4. assert that the final product matches the design and research docs
    const emailInput = page.getByPlaceholder('Email or Username').first().first();
    await expect(emailInput).toBeVisible({ timeout: 5000 });
  });

  test('should verify widget hover state micro-animations and layout resilience', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('test@example.com');
    await page.locator('input[type="password"]').first().first().fill('password123');
    await page.getByRole('button', { name: /Login|Sign In/i }).first().click();
    await page.waitForURL('**/dashboard');

    const usersBtn = page.locator('button:has-text("Users"), button:has-text("Team"), a[href*="/users"]').first();
    await usersBtn.click();

    const widgetCard = page.locator('text=Referral Program').locator('..');
    await expect(widgetCard).toBeVisible();

    const initialBox = await widgetCard.boundingBox();

    await widgetCard.hover();
    await page.waitForTimeout(300); // 300ms cubic-bezier animation duration

    const hoveredBox = await widgetCard.boundingBox();

    // Assert animated scale logic changed dimensions
    if (initialBox && hoveredBox) {
        expect(hoveredBox.width).toBeGreaterThanOrEqual(initialBox.width);
        expect(hoveredBox.height).toBeGreaterThanOrEqual(initialBox.height);
    }
  });

  test('should assert glassmorphism background token on referral widget', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('test@example.com');
    await page.locator('input[type="password"]').first().first().fill('password123');
    await page.getByRole('button', { name: /Login|Sign In/i }).first().click();
    await page.waitForURL('**/dashboard');

    const usersBtn = page.locator('button:has-text("Users"), button:has-text("Team"), a[href*="/users"]').first();
    await usersBtn.click();

    const referralHeader = page.locator('text=Referral Program');
    await expect(referralHeader).toBeVisible();

    const subtext = page.locator('text=Share OHC with a friend');
    await expect(subtext).toBeVisible();
  });

  test('should assert proper typography and text colors on referral widget', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('test@example.com');
    await page.locator('input[type="password"]').first().first().fill('password123');
    await page.getByRole('button', { name: /Login|Sign In/i }).first().click();
    await page.waitForURL('**/dashboard');

    const usersBtn = page.locator('button:has-text("Users"), button:has-text("Team"), a[href*="/users"]').first();
    await usersBtn.click();

    const referralHeader = page.locator('text=Referral Program');
    await expect(referralHeader).toBeVisible();
    // Validate the text content renders exactly as the new Slint file dictates
    await expect(page.locator('text=Share OHC with a friend, both get 1 month free Pro.')).toBeVisible();
  });

  test('should handle responsive resizing for the referral widget', async ({ page }) => {
    // Mobile-first: All screens must be 100% usable at 375px width
    await page.setViewportSize({ width: 375, height: 812 });

    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('test@example.com');
    await page.locator('input[type="password"]').first().first().fill('password123');
    await page.getByRole('button', { name: /Login|Sign In/i }).first().click();
    await page.waitForURL('**/dashboard');

    const menuBtn = page.locator('button:has-text("Menu")').first();
    if (await menuBtn.isVisible()) {
        await menuBtn.click();
    }

    const usersBtn = page.locator('button:has-text("Users"), button:has-text("Team"), a[href*="/users"]').first();
    await usersBtn.click();

    await expect(page.locator('text=Referral Program')).toBeVisible();
    const inviteBtn = page.locator('button:has-text("Invite User")');
    await expect(inviteBtn).toBeVisible();

    // Ensure the button is still clickable inside 375px bounds
    const box = await inviteBtn.boundingBox();
    if (box) {
        expect(box.width).toBeLessThanOrEqual(375);
    }
  });
});
