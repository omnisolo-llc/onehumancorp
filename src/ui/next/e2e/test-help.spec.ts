import { test, expect } from '@playwright/test';

test('Verify help center and navigation', async ({ page }) => {
  // Check Help Center
<<<<<<< HEAD
  await page.goto('/help');
=======
  await page.goto('http://localhost:3000/help');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'help-center.png' });
  await expect(page.locator('h1')).toContainText('Help Center');

  // Check specific article
<<<<<<< HEAD
  await page.goto('/help/getting-started-1');
=======
  await page.goto('http://localhost:3000/help/getting-started');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'help-article.png' });
  await expect(page.locator('h1')).toContainText('Getting Started with Your Store');

  // Check API Docs
<<<<<<< HEAD
  await page.goto('/api-docs');
=======
  await page.goto('http://localhost:3000/api-docs');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
  await page.waitForTimeout(1000);
  await expect(page.locator('text=Advanced:')).toBeVisible();

  // Check Changelog
<<<<<<< HEAD
  await page.goto('/changelog');
=======
  await page.goto('http://localhost:3000/changelog');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
  await page.waitForTimeout(1000);
  await expect(page.locator('h1')).toContainText('Release Notes & Changelog');
});
