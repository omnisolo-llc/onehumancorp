import { test, expect } from '@playwright/test';

test('Approve unified intake proposed task', async ({ page }) => {
  // Mock login and go to triage page
  await page.goto('http://localhost:3000/triage');

  // Simulate incoming webhook
  await page.click('text=Simulate Inquiry');

  // Wait for the simulated job to finish and refresh the list
  await page.waitForTimeout(4000);

  // Check if a card appears
  await expect(page.locator('text=New Lead: Maya')).toBeVisible();

  // Click Approve
  await page.click('text=Approve & Send');

  // Wait for the UI to update and remove the card
  await page.waitForTimeout(1000);
  await expect(page.locator('text=New Lead: Maya')).not.toBeVisible();
});
