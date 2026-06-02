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

test('Verify search and open video tutorial journey', async ({ page }) => {
  // Test search functionality in the Help Center
  await page.goto('http://localhost:3000/help');
  await page.waitForTimeout(1000);

  // Fill the search input
  await page.fill('input[placeholder="Search for help articles..."]', 'Getting Started');
  await page.waitForTimeout(500);

  // Verify that the filtered article is visible
  await expect(page.locator('text=Getting Started').first()).toBeVisible();

  // Click on the article
  await page.click('text=Getting Started');
  await page.waitForTimeout(1000);
  await expect(page.locator('h1')).toContainText('Getting Started with Your Store');
});

test('Verify HelpWidget videos tab and playing video', async ({ page }) => {
  // We go to any page that has the HelpWidget, which should be everywhere, e.g. root
  await page.goto('http://localhost:3000/');
  await page.waitForTimeout(1000);

  // Open HelpWidget
  await page.locator('button[aria-label="Help"]').first().click({ force: true });

  await page.waitForTimeout(500);

  // Switch to videos tab
  await page.click('button:has-text("Videos")', { force: true });
  await page.waitForTimeout(1000);

  // Click the first video to play
  await page.click('text=How to set up your first store easily');
  await page.waitForTimeout(500);

  // The video player modal should be visible (indicated by finding the title again inside the modal, or the fake video player UI)
  // Just checking if we can see the time indicator 0:00
  await expect(page.locator('text=0:00')).toBeVisible();

  // Close the video modal
  // The close button is a button with a specific SVG, we can try to hit escape or find the button
  await page.keyboard.press('Escape'); // Try escape if handled, else we need to click the close button

  // Click close button explicitly
  const closeBtn = page.locator('.fixed.inset-0.z-\\[100\\] button').first();
  if (await closeBtn.isVisible()) {
      await closeBtn.click();
  }
});
