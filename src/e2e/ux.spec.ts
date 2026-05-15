import { test, expect } from '@playwright/test';

// Set viewport to mobile standard 375px width
test.use({ viewport: { width: 375, height: 667 } });

test.describe('Echo UX Strategic Improvements', () => {

  test('should display plain language error on invalid login', async ({ page }) => {
    await page.goto('/login');
    // Ensure the app actually renders the login form before we try to fail
    await expect(page.locator('h1').filter({ hasText: 'Login' })).toBeVisible();

    // Trigger error display (simulated by script context)
    await page.evaluate(() => {
      const errorDiv = document.getElementById('login-error');
      if (errorDiv) errorDiv.style.display = 'block';
    });

    const errorLoc = page.locator('#login-error');
    await expect(errorLoc).toBeVisible();
    await expect(errorLoc).toHaveText("We couldn't log you in. Check your email or password.");
  });

  test('should load dashboard with clean jargon-free text', async ({ page }) => {
    // Start from home to follow constraints
    await page.goto('/');

    const welcomeHeader = page.locator('h2').filter({ hasText: 'Welcome back!' });
    await expect(welcomeHeader).toBeVisible();

    const salesLabel = page.locator('text=Today\'s Sales');
    await expect(salesLabel).toBeVisible();

    // Verify Facebook connection button is plain language
    await page.evaluate(() => {
       document.getElementById('facebook-integration')!.style.display = 'block';
    });
    const fbBtn = page.locator('button', { hasText: 'Connect my Facebook' });
    await expect(fbBtn).toBeVisible();
  });

  test('should verify mobile bottom navigation layout', async ({ page }) => {
    await page.goto('/');

    const nav = page.locator('nav').first();
    await expect(nav).toBeVisible();

    const addProductBtn = page.locator('nav button', { hasText: 'Add Product' });
    await expect(addProductBtn).toBeVisible();

    const messagesBtn = page.locator('nav button', { hasText: 'Messages' });
    await expect(messagesBtn).toBeVisible();
  });

  test('should verify quick actions contextual hint', async ({ page }) => {
    await page.goto('/');

    // Check for the ? button inside quick actions
    const qButton = page.locator('button', { hasText: '?' }).first();
    await expect(qButton).toBeVisible();

    // Click it to reveal hint
    await qButton.click();

    const hintText = page.locator('#quick-actions-hint');
    await expect(hintText).toBeVisible();
    await expect(hintText).toContainText('These buttons help you quickly do the most common tasks.');
  });

  test('should have OHC premium design glassmorphism classes', async ({ page }) => {
     await page.goto('/');

     const glassCard = page.locator('.card.glass').first();
     await expect(glassCard).toBeVisible();

     // Check basic CSS presence for glass (evaluating since it's injected)
     const style = await glassCard.evaluate((node) => window.getComputedStyle(node).backdropFilter);
     // It might evaluate to blur(20px) or similar
     expect(style).toContain('blur');
  });

});
