import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Viral Giveaway Generator', () => {
  test('should allow configuring a giveaway and hitting the pro paywall', async ({ page }) => {
    // Navigate to the dashboard and then to the generator
    await page.goto('/dashboard');

    // Wait for the app shell to load
    await expect(page.locator('.app-shell')).toBeVisible();

    // Click the Giveaway Generator link
    await page.click('text="Viral Giveaway Generator"');

    // Wait for the page to load
    await expect(page.locator('h1', { hasText: 'Viral Giveaway Generator' })).toBeVisible();

    // Verify the preview contains the default title
    await expect(page.locator('h3', { hasText: 'Win a Free Custom Cake!' })).toBeVisible();

    // Change the title
    const titleInput = page.locator("text=Giveaway Headline").locator("..").locator("input");
    await titleInput.fill('Win a Year of Coffee!');

    // Verify the preview updates
    await expect(page.locator('h3', { hasText: 'Win a Year of Coffee!' })).toBeVisible();

    // Change the prize
    const prizeInput = page.locator("text=Prize Description").locator("..").locator("input");
    await prizeInput.fill('365 Days of Free Coffee (Value $1000)');

    // Verify the preview updates
    await expect(page.locator('p', { hasText: 'Prize: 365 Days of Free Coffee (Value $1000)' })).toBeVisible();

    // Try to remove branding without Pro
    const brandingCheckbox = page.locator('label', { hasText: 'Remove "Powered by OHC" Badge' });
    await brandingCheckbox.click();

    // Paywall should appear
    await expect(page.locator('h2', { hasText: 'Make it 100% Yours' })).toBeVisible();
    await expect(page.locator('text="Upgrade to Pro"')).toBeVisible();

    // Close the paywall
    await page.click('text="Keep Branding"');

    // Verify paywall is gone
    await expect(page.locator('h2', { hasText: 'Make it 100% Yours' })).toBeHidden();
  });
});
