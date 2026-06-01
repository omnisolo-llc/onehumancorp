import { test, expect } from './fixtures';

test.describe('Help Features', () => {
  test('Help Center page and search functionality', async ({ page }) => {
    await page.goto('/help');
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();
    await expect(page.getByText('Getting Started')).toBeVisible();
    await expect(page.getByText('My Store')).toBeVisible();
    const searchInput = page.getByPlaceholder('Search for help articles...');
    await searchInput.fill('Getting Paid');
    await expect(page.getByText('Getting Paid')).toBeVisible();
    await expect(page.getByText('My Store')).toBeHidden();
    await searchInput.fill('');
    await page.waitForTimeout(500);
    await page.getByText('Getting Started').click();
    await expect(page.getByRole('heading', { name: 'Getting Started with Your Store' })).toBeVisible();
    await expect(page.getByText('Step 1: Tell us about your business')).toBeVisible();
    await page.getByRole('button', { name: '← Back to Help Center' }).click();
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();
  });

  test('API Documentation page', async ({ page }) => {
    await page.goto('/api-docs');
    await expect(page.getByText('Advanced:')).toBeVisible();
    await expect(page.getByText('This section is for developers directly integrating with our APIs.')).toBeVisible();
  });

  test('Changelog and Release Notes page', async ({ page }) => {
    await page.goto('/changelog');
    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.getByText('Version 1.0 (Latest)')).toBeVisible();
    await expect(page.getByText('Interactive AI Store Builder:')).toBeVisible();
    await expect(page.getByText('Smart Tooltips:')).toBeVisible();
  });
});

test.describe('KAIROS Walkthrough', () => {
  test('navigates through the walkthrough steps', async ({ page }) => {
    await page.goto('/kairos?walkthrough=true');
    await page.waitForTimeout(1500);
    const firstBubbleText = page.getByText("The Shared Task List is the 'Brain'");
    await expect(firstBubbleText).toBeVisible();
    await page.getByRole('button', { name: 'Next' }).click();
    const secondBubbleText = page.getByText("The Teammate Mesh acts as the 'Nerves'");
    await expect(secondBubbleText).toBeVisible();
    await page.getByRole('button', { name: 'Next' }).click();
    const thirdBubbleText = page.getByText("AutoDream is the 'Memory'");
    await expect(thirdBubbleText).toBeVisible();
    await page.getByRole('button', { name: 'Finish' }).click();
    await expect(thirdBubbleText).toBeHidden();
  });
});
