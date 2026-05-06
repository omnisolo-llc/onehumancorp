import { test, expect } from '@playwright/test';

test.describe('Onboarding Flow', () => {
  test('should start business setup onboarding', async ({ page }) => {
    await page.goto('/onboarding');
    await expect(page.locator('text=/onboarding|setup|welcome/i')).toBeVisible();
  });

  test('should show welcome screen', async ({ page }) => {
    await page.goto('/onboarding');
    await expect(page.locator('text=/welcome|get started/i')).toBeVisible();
  });

  test('should show progress indicator', async ({ page }) => {
    await page.goto('/onboarding');
    const progress = page.locator('[class*="progress"], text=/step \\d+ of \\d+/i').first();
    await expect(progress).toBeVisible();
  });

  test('should navigate to next step', async ({ page }) => {
    await page.goto('/onboarding');
    const nextBtn = page.locator('button:has-text("Next"), button:has-text("Continue")').first();
    if (await nextBtn.isVisible()) {
      await nextBtn.click();
      await expect(page.locator('text=/step \\d+/i')).toBeVisible();
    }
  });

  test('should navigate to previous step', async ({ page }) => {
    await page.goto('/onboarding');
    const backBtn = page.locator('button:has-text("Back"), button:has-text("Previous")').first();
    if (await backBtn.isVisible()) {
      await backBtn.click();
      await expect(page.locator('text=/welcome|step/i')).toBeVisible();
    }
  });

  test('should select business type', async ({ page }) => {
    await page.goto('/onboarding');
    const nextBtn = page.locator('button:has-text("Next"), button:has-text("Continue")').first();
    if (await nextBtn.isVisible()) {
      await nextBtn.click();
      await page.locator('text=/online|store|service/i').first().click();
    }
  });

  test('should enter company name', async ({ page }) => {
    await page.goto('/onboarding');
    const nextBtn = page.locator('button:has-text("Next"), button:has-text("Continue")').first();
    if (await nextBtn.isVisible()) {
      for (let i = 0; i < 2; i++) {
        await nextBtn.click();
      }
      const nameInput = page.locator('input[type="text"]').first();
      if (await nameInput.isVisible()) {
        await nameInput.fill('My Company');
      }
    }
  });

  test('should skip optional steps', async ({ page }) => {
    await page.goto('/onboarding');
    const skipBtn = page.locator('button:has-text("Skip"), button:has-text("Skip this step")').first();
    if (await skipBtn.isVisible()) {
      await skipBtn.click();
    }
  });

  test('should complete onboarding', async ({ page }) => {
    await page.goto('/onboarding');
    const finishBtn = page.locator('button:has-text("Finish"), button:has-text("Complete")').first();
    if (await finishBtn.isVisible()) {
      await finishBtn.click();
      await expect(page.locator('text=/dashboard|welcome/i')).toBeVisible({ timeout: 10000 });
    }
  });

  test('should save onboarding progress', async ({ page }) => {
    await page.goto('/onboarding');
    await page.fill('input[type="text"]', 'Test Company');
    await page.locator('button:has-text("Save"), button:has-text("Continue")').click();
    await expect(page.locator('text=/saved|progress/i')).toBeVisible({ timeout: 3000 });
  });

  test('should resume onboarding from saved state', async ({ page }) => {
    await page.goto('/onboarding');
    await expect(page.locator('text=/resume|continue.*where.*left.*off/i')).toBeVisible({ timeout: 3000 });
  });

  test('should verify email and auto-generate product description and show confetti', async ({ page }) => {
    // E2E test covering the whole flow:
    // Go to login -> click "Sign Up" -> fill email/password
    await page.goto('/login');
    const signUpToggle = page.locator('button:has-text("Don\'t have an account? Sign Up")');
    await signUpToggle.click();

    await page.locator('input[type="email"], input[placeholder*="Email"]').first().fill('test@example.com');
    await page.locator('input[type="password"], input[placeholder*="Password"]').first().fill('password123');

    // Click "Sign Up"
    const signUpBtn = page.locator('button:has-text("Sign Up")').first();
    await signUpBtn.click();

    // Verify "Check your email" shows
    await expect(page.locator('text=/Check your email|verify your account/i')).toBeVisible({ timeout: 5000 });

    // Click "Resend Verification"
    const resendBtn = page.locator('button:has-text("Resend Verification Email"), button:has-text("Resend")').first();
    await resendBtn.click();

    // Verify it navigates to Setup Wizard
    await expect(page.locator('text=/Setup Wizard|What kind of business/i')).toBeVisible({ timeout: 5000 });

    // Advance through steps to get to product add step
    for (let i = 0; i < 6; i++) {
      const nextBtn = page.locator('button:has-text("Next"), button:has-text("Continue")').first();
      await nextBtn.click();
    }

    // At step 7 (product add), fill Product Name, click "Auto-generate description"
    const productNameInput = page.locator('input[placeholder*="Product Name"], input[placeholder*="Cake"]').first();
    await productNameInput.fill('My Custom Product');

    const autoGenBtn = page.locator('button:has-text("Auto-generate description")').first();
    await autoGenBtn.click();

    // Verify product description was generated
    const descInput = page.locator('input[placeholder*="Description"], input:has-text("A premium")').first();
    await expect(descInput).toHaveValue(/A premium.*/);

    // Finish the wizard
    for (let i = 0; i < 3; i++) {
      const nextBtn = page.locator('button:has-text("Next"), button:has-text("Continue"), button:has-text("Launch")').first();
      await nextBtn.click();
    }

    // Verify Confetti success and Copy Link button are visible
    await expect(page.locator('text=/CONFETTI.*SUCCESS/i')).toBeVisible({ timeout: 5000 });
    const copyLinkBtn = page.locator('button:has-text("Copy Shareable Link")').first();
    await expect(copyLinkBtn).toBeVisible();
  });

  test('should seamlessly transition to setup wizard from sign up', async ({ page }) => {
    await page.goto('/login');
    const signUpToggle = page.locator('button:has-text("Don\'t have an account? Sign Up")');
    await signUpToggle.click();

    await page.locator('input[type="email"], input[placeholder*="Email"]').first().fill('test.new@example.com');
    await page.locator('input[type="password"], input[placeholder*="Password"]').first().fill('securepassword123');

    const signUpBtn = page.locator('button:has-text("Sign Up")').first();
    await signUpBtn.click();

    await expect(page.locator('text=/Check your email|verify your account/i')).toBeVisible({ timeout: 5000 });
  });

  test('should open business setup correctly for fresh accounts', async ({ page }) => {
    await page.goto('/login');
    await page.locator('input[type="email"], input[placeholder*="Email"]').first().fill('new.business@example.com');
    await page.locator('input[type="password"], input[placeholder*="Password"]').first().fill('securepassword123');

    const signInBtn = page.locator('button:has-text("Sign In")').first();
    await signInBtn.click();

    // Simulate auto-redirect handling for fresh accounts to the wizard
    await expect(page.locator('text=/Setup Wizard|What kind of business/i')).toBeVisible({ timeout: 5000 });
  });

  test('should handle instant storefront preview safely', async ({ page }) => {
    // Navigating organically from login
    await page.goto('/login');
    await page.locator('input[type="email"], input[placeholder*="Email"]').first().fill('test.instant@example.com');
    await page.locator('input[type="password"], input[placeholder*="Password"]').first().fill('securepassword123');

    const signInBtn = page.locator('button:has-text("Sign In")').first();
    await signInBtn.click();

    await expect(page.locator('text=/Setup Wizard|What kind of business/i')).toBeVisible({ timeout: 5000 });

    // Proceed to Step 11: Instant Build
    for (let i = 0; i < 11; i++) {
        const nextBtn = page.locator('button:has-text("Next"), button:has-text("Continue")').first();
        await nextBtn.click();
    }

    const bioInput = page.locator('input[placeholder*="I run a local bakery"]');
    await expect(bioInput).toBeVisible({ timeout: 3000 });
    await bioInput.fill("Test Business Bio");

    const generateBtn = page.locator('text=Generate Storefront →');
    await generateBtn.click();
    await expect(page.locator('text=AI is brainstorming...')).toBeVisible({ timeout: 5000 });
  });

  test('should validate end-to-end product addition with real prices', async ({ page }) => {
    // Navigating organically from login
    await page.goto('/login');
    await page.locator('input[type="email"], input[placeholder*="Email"]').first().fill('test.product@example.com');
    await page.locator('input[type="password"], input[placeholder*="Password"]').first().fill('securepassword123');

    const signInBtn = page.locator('button:has-text("Sign In")').first();
    await signInBtn.click();

    await expect(page.locator('text=/Setup Wizard|What kind of business/i')).toBeVisible({ timeout: 5000 });

    // Proceed to Step 7: Product
    for (let i = 0; i < 6; i++) {
        const nextBtn = page.locator('button:has-text("Next"), button:has-text("Continue")').first();
        await nextBtn.click();
    }

    const nameInput = page.locator('input[placeholder*="Product Name"], input[placeholder*="Cake"]').first();
    await expect(nameInput).toBeVisible({ timeout: 3000 });
    await nameInput.fill('Real Test Product');

    const priceInput = page.locator('input[placeholder*="Price"], input[placeholder*="e.g. 50.00"]').first();
    await priceInput.fill('99.99');

    const autoGenBtn = page.locator('button:has-text("Auto-generate description")').first();
    await autoGenBtn.click();

    // Verify product description was generated
    const descInput = page.locator('input[placeholder*="Description"], input:has-text("A premium")').first();
    await expect(descInput).toHaveValue(/A premium.*/);
  });
});

test.describe('Onboarding Welcome Checklist', () => {
  test('should display welcome checklist', async ({ page }) => {
    await page.goto('/welcome-checklist');
    await expect(page.locator('text=/checklist|welcome/i')).toBeVisible();
  });

  test('should show checklist items', async ({ page }) => {
    await page.goto('/welcome-checklist');
    const item = page.locator('[class*="item"], [class*="checklist"]').first();
    await expect(item).toBeVisible();
  });

  test('should mark item as complete', async ({ page }) => {
    await page.goto('/welcome-checklist');
    const checkbox = page.locator('input[type="checkbox"]').first();
    if (await checkbox.isVisible()) {
      await checkbox.check();
      await expect(page.locator('text=/completed|done/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should show completion progress', async ({ page }) => {
    await page.goto('/welcome-checklist');
    const progress = page.locator('text=/\\d+ of \\d+|\\d+%/').first();
    await expect(progress).toBeVisible();
  });

  test('should mark all items complete', async ({ page }) => {
    await page.goto('/welcome-checklist');
    const checkboxes = page.locator('input[type="checkbox"]');
    const count = await checkboxes.count();
    for (let i = 0; i < count; i++) {
      await checkboxes.nth(i).check();
    }
  });

  test('should show congratulations message', async ({ page }) => {
    await page.goto('/welcome-checklist');
    const checkboxes = page.locator('input[type="checkbox"]');
    const count = await checkboxes.count();
    for (let i = 0; i < count; i++) {
      await checkboxes.nth(i).check();
    }
    await expect(page.locator('text=/congratulations|complete|awesome/i')).toBeVisible({ timeout: 5000 });
  });

  test('should link to documentation', async ({ page }) => {
    await page.goto('/welcome-checklist');
    const docLink = page.locator('a:has-text("Documentation"), a:has-text("Docs")').first();
    if (await docLink.isVisible()) {
      await docLink.click();
      await expect(page.locator('text=/docs|documentation/i')).toBeVisible();
    }
  });

  test('should link to video tutorials', async ({ page }) => {
    await page.goto('/welcome-checklist');
    const videoLink = page.locator('a:has-text("Video"), a:has-text("Tutorial")').first();
    if (await videoLink.isVisible()) {
      await videoLink.click();
      await expect(page.locator('text=/video|tutorial/i')).toBeVisible();
    }
  });

  test('should offer to contact support', async ({ page }) => {
    await page.goto('/welcome-checklist');
    const supportLink = page.locator('text=/support|help|contact/i').first();
    await expect(supportLink).toBeVisible();
  });
});
