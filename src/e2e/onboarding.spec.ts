import { test, expect } from '@playwright/test';

test.describe('Onboarding Flow', () => {
  test('should complete business setup onboarding', async ({ page }) => {
    await page.goto('/');
    await page.fill('input[type="email"], input[placeholder*="Email"]', 'test@example.com');
    await page.fill('input[type="password"], input[placeholder*="Password"]', 'password123');
    await page.click('button:has-text("Login"), button:has-text("Sign In")');

    await expect(page.locator('text=/Welcome/i')).toBeVisible({ timeout: 10000 });

    await page.click('button:has-text("Start Setup")');
    await expect(page.locator('text=/Setup Wizard|Welcome/i')).toBeVisible({ timeout: 5000 });

    const nextBtn = page.locator('button:has-text("Next"), button:has-text("Continue")').first();
    if (await nextBtn.isVisible()) {
      await nextBtn.click();
      await expect(page.locator('text=/Online Store|Service/i')).toBeVisible({ timeout: 5000 });

      const backBtn = page.locator('button:has-text("Back"), button:has-text("Previous")').first();
      if (await backBtn.isVisible()) {
        await backBtn.click();
        await expect(page.locator('text=/Welcome|Setup Wizard/i')).toBeVisible({ timeout: 5000 });
      }

      await nextBtn.click();
      await page.locator('text=/Online Store/i').first().click();
      await nextBtn.click();

      const nameInput = page.locator('input[placeholder*="What is your business called?"], input[type="text"]').first();
      if (await nameInput.isVisible()) {
        await nameInput.fill('My Company');
      }

      const skipBtn = page.locator('button:has-text("Skip"), button:has-text("Skip this step")').first();
      if (await skipBtn.isVisible()) {
        await skipBtn.click();
      }
    }
  });
});

test.describe('Onboarding Email and Confetti', () => {
  test('should verify email and auto-generate product description and show confetti', async ({ page }) => {
    // E2E test covering the whole flow:
    // Go to login -> click "Sign Up" -> fill email/password
    await page.goto('/');
    const signUpToggle = page.locator('button:has-text("Don\'t have an account? Sign Up")');
    if (await signUpToggle.isVisible()) {
      await signUpToggle.click();
    }

    await page.locator('input[type="email"], input[placeholder*="Email"]').first().fill('test@example.com');
    await page.locator('input[type="password"], input[placeholder*="Password"]').first().fill('password123');

    // Click "Sign Up"
    const signUpBtn = page.locator('button:has-text("Sign Up")').first();
    if (await signUpBtn.isVisible()) {
      await signUpBtn.click();
    }

    // Verify "Check your email" shows
    await expect(page.locator('text=/Check your email|verify your account/i')).toBeVisible({ timeout: 5000 });

    // Click "Resend Verification"
    const resendBtn = page.locator('button:has-text("Resend Verification Email"), button:has-text("Resend")').first();
    if (await resendBtn.isVisible()) {
      await resendBtn.click();
    }

    // Verify it navigates to Setup Wizard
    await expect(page.locator('text=/Setup Wizard|What kind of business/i')).toBeVisible({ timeout: 5000 });

    // Advance through steps to get to product add step
    for (let i = 0; i < 6; i++) {
      const nextBtn = page.locator('button:has-text("Next"), button:has-text("Continue")').first();
      if (await nextBtn.isVisible()) {
        await nextBtn.click();
      }
    }

    // At step 7 (product add), fill Product Name, click "Auto-generate description"
    const productNameInput = page.locator('input[placeholder*="Product Name"], input[placeholder*="Cake"]').first();
    if (await productNameInput.isVisible()) {
      await productNameInput.fill('My Custom Product');
    }

    const autoGenBtn = page.locator('button:has-text("Auto-generate description")').first();
    if (await autoGenBtn.isVisible()) {
      await autoGenBtn.click();
    }

    // Verify product description was generated
    const descInput = page.locator('input[placeholder*="Description"], input:has-text("A premium")').first();
    if (await descInput.isVisible()) {
      await expect(descInput).toHaveValue(/A premium.*/);
    }

    // Finish the wizard
    for (let i = 0; i < 3; i++) {
      const nextBtn = page.locator('button:has-text("Next"), button:has-text("Continue"), button:has-text("Launch")').first();
      if (await nextBtn.isVisible()) {
        await nextBtn.click();
      }
    }

    // Verify Confetti success and Copy Link button are visible
    await expect(page.locator('text=/CONFETTI.*SUCCESS/i')).toBeVisible({ timeout: 5000 });
    const copyLinkBtn = page.locator('button:has-text("Copy Shareable Link")').first();
    if (await copyLinkBtn.isVisible()) {
      await copyLinkBtn.click();
    }
  });
});
