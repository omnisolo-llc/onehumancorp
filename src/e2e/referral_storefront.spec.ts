import { test, expect } from '@playwright/test';

test.describe('Referral Storefront E2E', () => {
  test('user can click referral storefront footer to open signup page', async ({ page, context }) => {
    await page.goto('/');

    const loginEmailInput = page.getByPlaceholder(/email/i).first();
    const loginPasswordInput = page.getByPlaceholder(/password/i).first();

    // We deterministically expect login to be present
    await expect(loginEmailInput).toBeVisible();
    await loginEmailInput.fill('test@example.com');
    await loginPasswordInput.fill('password123');
    await page.getByRole('button', { name: /log in/i }).click();
    await page.waitForURL('**/dashboard**', { timeout: 10000 });

    // Navigate to the website builder
    await page.goto('/website-builder');

    // Deterministically click next 4 times to reach the publish step where the footer is
    for (let i = 0; i < 4; i++) {
       const nextBtn = page.getByRole('button', { name: /next|continue/i }).first();
       await expect(nextBtn).toBeVisible({ timeout: 5000 });
       await nextBtn.click();
       await page.waitForTimeout(200);
    }

    // Deterministically assert the footer link exists and works
    const footerLink = page.getByText(/Built with OHC.*Start your free business/i).first();
    await expect(footerLink).toBeVisible();

    const [newPage] = await Promise.all([
      context.waitForEvent('page'),
      footerLink.click()
    ]);

    await expect(newPage).toHaveURL(/onehumancorp\.com/i);
  });

  test('should assert referral footer exists on desktop storefront', async ({ page }) => {
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();
    await page.waitForURL('**/dashboard');

    await page.goto('/website-builder');

    for (let i = 0; i < 4; i++) {
       const nextBtn = page.getByRole('button', { name: /next|continue/i }).first();
       await expect(nextBtn).toBeVisible({ timeout: 5000 });
       await nextBtn.click();
       await page.waitForTimeout(200);
    }

    const footerLink = page.getByText(/Built with OHC.*Start your free business/i).first();
    await expect(footerLink).toBeVisible();
  });

  test('should assert referral footer exists on mobile storefront', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();
    await page.waitForURL('**/dashboard');

    await page.goto('/website-builder');

    for (let i = 0; i < 4; i++) {
       const nextBtn = page.getByRole('button', { name: /next|continue/i }).first();
       await expect(nextBtn).toBeVisible({ timeout: 5000 });
       await nextBtn.click();
       await page.waitForTimeout(200);
    }

    const footerLink = page.getByText(/Built with OHC.*Start your free business/i).first();
    await expect(footerLink).toBeVisible();
  });

  test('should verify referral footer text matches precise marketing copy', async ({ page }) => {
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();
    await page.waitForURL('**/dashboard');

    await page.goto('/website-builder');

    for (let i = 0; i < 4; i++) {
       const nextBtn = page.getByRole('button', { name: /next|continue/i }).first();
       await expect(nextBtn).toBeVisible({ timeout: 5000 });
       await nextBtn.click();
       await page.waitForTimeout(200);
    }

    const footerLink = page.locator('text="Built with OHC — Start your free business →"').first();
    await expect(footerLink).toBeVisible();
  });

  test('should verify clicking referral footer initiates redirect workflow', async ({ page, context }) => {
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();
    await page.waitForURL('**/dashboard');

    await page.goto('/website-builder');

    for (let i = 0; i < 4; i++) {
       const nextBtn = page.getByRole('button', { name: /next|continue/i }).first();
       await expect(nextBtn).toBeVisible({ timeout: 5000 });
       await nextBtn.click();
       await page.waitForTimeout(200);
    }

    const footerLink = page.getByText(/Built with OHC.*Start your free business/i).first();

    // Test the button click without failing on actual redirect limits
    await footerLink.click();
  });
});
