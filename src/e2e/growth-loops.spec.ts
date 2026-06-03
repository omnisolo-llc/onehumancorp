import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Growth Loops', () => {
  test('Milestones page renders and is interactive', async ({ page, adminPage }) => {
    // Note: The UI runs in legacy mode on 3000 but the playwright harness
    // normally serves Tauri UI. For this E2E we'll just test the URL mapping
    // or if the page works if we navigate to it.
    await page.goto('/milestones');
    await expect(page.getByText('Success Milestones')).toBeVisible();

    // Select the 10th order milestone
    await page.getByText('10th Order Milestone').first().click();

    // Verify the preview updates
    await expect(page.getByText('Double digits! Your business is gaining momentum.')).toBeVisible();

    // Check share message button
    await expect(page.getByText('Copy Share Message')).toBeVisible();
  });

  test('Share Cards page allows customization', async ({ page, adminPage }) => {
    await page.goto('/share-cards');
    await expect(page.getByText('Create Share Card')).toBeVisible();

    // Customize
    await page.fill('input[type="text"]', 'Maya Cakes');
    await page.fill('textarea', 'The best vegan cakes in town');

    // Verify preview
    await expect(page.getByText('Maya Cakes').first()).toBeVisible();
    await expect(page.getByText('The best vegan cakes in town').first()).toBeVisible();
  });
});
