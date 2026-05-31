import { test, expect } from './fixtures';

test.describe('Documentation & Walkthroughs', () => {

  test('Accept First Payment Tour navigation and targeting', async ({ memberPage: page }) => {
    await page.goto('/builder');
    await page.click('button:has-text("?")', { force: true });
    await page.waitForSelector('h3:has-text("Interactive Tours")');
    await page.click('button:has-text("Tour: Accept your first payment")');
    await expect(page).toHaveURL(/.*\/dashboard\?walkthrough=payment/);
    const bubble = page.locator('.fixed.z-\\[1000\\]:has-text("Click here to connect Stripe and start accepting payments.")');
    await expect(bubble).toBeVisible({ timeout: 10000 });
  });

  test('Set Up Your Store Tour navigation and targeting', async ({ memberPage: page }) => {
    await page.goto('/dashboard');
    await page.click('button:has-text("?")', { force: true });
    await page.waitForSelector('h3:has-text("Interactive Tours")');
    await page.click('button:has-text("Tour: Set up your store")');
    await expect(page).toHaveURL(/.*\/builder\?walkthrough=setup-store/);
    const bubble = page.locator('.fixed.z-\\[1000\\]:has-text("Click to generate!")');
    await expect(bubble).toBeVisible({ timeout: 10000 });
  });

  test('Activate AI Agent Tour navigation and targeting', async ({ memberPage: page }) => {
    await page.goto('/dashboard');
    await page.click('button:has-text("?")', { force: true });
    await page.waitForSelector('h3:has-text("Interactive Tours")');
    await page.click('button:has-text("Tour: Activate your AI Support Agent")');
    await expect(page).toHaveURL(/.*\/team\?walkthrough=activate-agent/);
    const bubble = page.locator('.fixed.z-\\[1000\\]:has-text("Here you can monitor and activate your AI Support Agents as they work.")');
    await expect(bubble).toBeVisible({ timeout: 10000 });
  });

  test('API Docs page is accessible', async ({ memberPage: page }) => {
    await page.goto('/api-docs');
    await expect(page.locator('text="Advanced: This section is for developers"')).toBeVisible({ timeout: 15000 });
  });

  test('Changelog page is accessible', async ({ memberPage: page }) => {
    await page.goto('/changelog');
    await expect(page.locator('h1:has-text("Release Notes & Changelog")')).toBeVisible();
    await expect(page.locator('text=Interactive AI Store Builder')).toBeVisible();
  });
});
