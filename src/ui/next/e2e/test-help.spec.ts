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
  await expect(page.locator('h3:has-text("Help Agent")')).toBeVisible();
});

test('Verify Interactive Walkthrough triggers', async ({ page }) => {
  await page.goto('http://localhost:3000/kairos?walkthrough=true');
  await page.waitForTimeout(1000);

  // Wait for the walkthrough bubble which has the text "Finish" or "Next" or specific class
  // Checking for the speech bubble component: class includes "animate-pop-in"
  const walkthroughBubble = page.locator('.animate-pop-in').first();
  await expect(walkthroughBubble).toBeVisible();
});


test('Verify context tooltips', async ({ page }) => {
  await page.goto('http://localhost:3000/help');
  await page.waitForTimeout(1000);

  // The HelpCenter component has an Ask AI / Video etc tabs
  // Actually we need to be somewhere where a tooltip exists.
  // Wait, the "changelog-nav-tooltip" is shown inside the "whatsnew" tab of the HelpWidget
  await page.goto('http://localhost:3000/');
  await page.waitForTimeout(1000);

  // Click the help button to open the widget
  const helpButton = page.locator('button[aria-label="Help"]');
  await helpButton.click();

  // Hover over the Help button to trigger tooltip
  await helpButton.hover();

  // Because it fetches tooltips async, we need to wait a moment. The page is already loaded so the fetch might be done.
  await page.waitForTimeout(500);
  await helpButton.dispatchEvent('mouseenter'); // ensure it fires

  // The tooltip should appear with the text
  const tooltipText = page.locator('text=Need help? Click here for guides, videos, and to ask our AI.');
  await expect(tooltipText).toBeVisible();
});

test('Verify Video Tutorials', async ({ page }) => {
  await page.goto('http://localhost:3000/');
  await page.waitForTimeout(1000);

  // Click the help button to open the widget
  const helpButton = page.locator('button[aria-label="Help"]');
  await helpButton.click();

  const helpWidget = page.locator('#help-widget-container.fixed');
  await expect(helpWidget).toBeVisible();

  const videosTab = helpWidget.locator('button:has-text("Videos")').first();
  await videosTab.click();

  // Click first video
  const firstVideo = page.locator('text=How to set up your first store easily').first();
  await expect(firstVideo).toBeVisible();
  await firstVideo.click();

  // Video player modal should appear
  const videoPlayerModal = page.locator('.fixed.z-\\[100\\]');
  await expect(videoPlayerModal).toBeVisible();
});

test('Verify Help Center search', async ({ page }) => {
  await page.goto('http://localhost:3000/help');
  await page.waitForTimeout(1000);

  const searchInput = page.locator('input[placeholder="Search for help articles..."]');
  await expect(searchInput).toBeVisible();

  await searchInput.fill('Getting Paid');
  await page.waitForTimeout(500); // Give React state time to filter

  // "Getting Paid" card should be visible
  await expect(page.locator('h2', { hasText: 'Getting Paid' })).toBeVisible();

  // "Getting Started" card should NOT be visible
  await expect(page.locator('h2', { hasText: 'Getting Started' })).not.toBeVisible();
});
