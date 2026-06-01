import { test, expect } from '@playwright/test';

test('Verify help center and navigation', async ({ page }) => {
  // Check Help Center
  await page.goto('http://localhost:3000/help');
  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'help-center.png' });
  await expect(page.locator('h1')).toContainText('Help Center');

  // Check specific article
  await page.goto('http://localhost:3000/help/getting-started');
  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'help-article.png' });
  await expect(page.locator('h1')).toContainText('Getting Started with Your Store');

  // Check API Docs
  await page.goto('http://localhost:3000/api-docs');
  await page.waitForTimeout(1000);
  await expect(page.locator('text=Advanced:')).toBeVisible();

  // Check Changelog
  await page.goto('http://localhost:3000/changelog');
  await page.waitForTimeout(1000);
  await expect(page.locator('h1')).toContainText('Release Notes & Changelog');
});

test('Verify HelpWidget and HelpChat UI', async ({ page }) => {
  await page.goto('http://localhost:3000/');
  await page.waitForTimeout(1000);

  // The HelpChat button "Ask anything" should be in the layout
  const askAnythingBtn = page.locator('button', { hasText: 'Ask anything' });
  await expect(askAnythingBtn).toBeVisible();
  await askAnythingBtn.click();
  await expect(page.locator('text=Help Agent')).toBeVisible();
});

test('Verify Interactive Walkthrough triggers', async ({ page }) => {
  await page.goto('http://localhost:3000/kairos?walkthrough=true');
  await page.waitForTimeout(1000);

  // Wait for the walkthrough bubble which has the text "Finish" or "Next" or specific class
  // Checking for the speech bubble component: class includes "animate-pop-in"
  const walkthroughBubble = page.locator('.animate-pop-in').first();
  await expect(walkthroughBubble).toBeVisible();
});
