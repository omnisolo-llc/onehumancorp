import { test, expect } from '@playwright/test';

test.describe('Scribe: Documentation & Help CUJs', () => {
  test.beforeEach(async ({ page }) => {
    // Bypass onboarding by navigating directly and setting flag
    await page.goto('http://localhost:3000/dashboard');
    await page.evaluate(() => {
      localStorage.setItem('has_onboarded', 'true');
      localStorage.setItem('onboarding_completed', 'true');
    });
    // Reload to ensure state is picked up
    await page.goto('http://localhost:3000/help');
    await page.waitForLoadState('networkidle');
  });

  test('CUJ 1: Business Owner searches for help and reads an article', async ({ page }) => {
    // 1. Search for "Paid"
    const searchInput = page.getByPlaceholder('Search for help...');
    await searchInput.fill('Paid');

    // 2. Click on "Getting Paid" article
    const article = page.locator('text=Getting Paid');
    await expect(article).toBeVisible();
    await article.click();
  });

  test('CUJ 2: Business Owner asks the AI Support Agent a question', async ({ page }) => {
    // 1. Open Widget
    await page.click('button[aria-label="Help"]');

    // 2. Switch to Ask AI tab
    await page.click('button:has-text("Ask AI")');

    // 3. Type question
    await page.getByPlaceholder('Ask anything...').fill('How do I add a product?');
    await page.click('button[aria-label="Send message"]');

    // 4. Verify bot reply appears
    await expect(page.locator('div.bg-blue-50')).toContainText('product', { timeout: 10000 });
  });

  test('CUJ 3: Business Owner watches a tutorial video', async ({ page }) => {
    // 1. Open Widget
    await page.click('button[aria-label="Help"]');

    // 2. Switch to Videos tab
    await page.click('button:has-text("Videos")');

    // 3. Click first video
    await page.locator('div.grid-cols-2 > div').first().click();

    // 4. Verify video player modal is open
    await expect(page.locator('h3:has-text("How to")')).toBeVisible();

    // 5. Close video
    await page.click('button[aria-label="Close video"]');
    await expect(page.locator('button[aria-label="Close video"]')).not.toBeVisible();
  });

  test('CUJ 4: Business Owner completes the "Set up your store" walkthrough', async ({ page }) => {
    // 1. Open Widget
    await page.click('button[aria-label="Help"]');

    // 2. Start Tour
    await page.click('text=Tour: Set up your store');

    // 3. Verify walkthrough tooltip appears
    await expect(page.locator('text=Quick Guide')).toBeVisible();
  });

  test('CUJ 5: Business Owner views contextual tooltips', async ({ page }) => {
    // 1. Hover an element with a tooltip
    await page.hover('button[aria-label="Help"]');

    // 2. Verify tooltip text appears
    await expect(page.locator('text=Need help?')).toBeVisible();
  });
});
