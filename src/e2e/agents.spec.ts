import { test, expect } from './fixtures';

test.describe('Agent Management Full Journey', () => {
  // Test 1: Start at home page (dashboard), click the top navigation link `Agents`
  test('should navigate from home to agents page via top navigation', async ({ page }) => {
    // Starting at home page (fixtures logs us in to dashboard)
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // Click the top navigation link
    await page.getByRole('link', { name: 'Agents' }).click();

    // Assert the "Agents" heading is visible
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  // Test 2: Navigate to the agents screen and verify visual elements
  test('should verify agents screen visual elements and glassmorphism styling', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();

    // Verify card content
    const card = page.locator('.card.glass').first();
    await expect(card).toBeVisible();
    await expect(card.locator('h3', { hasText: 'Marketing Pro' })).toBeVisible();
    await expect(card.locator('p', { hasText: 'Status: Active' })).toBeVisible();

    // Verify Hire Agent button exists
    await expect(card.locator('button', { hasText: 'Hire Agent' })).toBeVisible();
  });

  // Test 3: Navigate to the agents screen, click the secondary back button `Back`
  test('should navigate from agents page back to dashboard via back button', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();

    // Click secondary Back button
    await page.getByRole('button', { name: 'Back' }).click();

    // Assert the "Dashboard" heading is visible
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  // Test 4: Start at home page (dashboard), click the mobile bottom nav button "Home"
  test('should navigate to dashboard via mobile bottom nav Home button', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // Click the mobile bottom nav button "Home"
    await page.locator('#mobile-bottom-nav').getByRole('button', { name: 'Home' }).click();

    // Assert the "Dashboard" heading is visible
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  // Test 5: Start at home page (dashboard), click the top navigation link "Connect Tools"
  test('should navigate to connect tools via top navigation', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // Click the "Connect Tools" link
    await page.getByRole('link', { name: 'Connect Tools' }).click();

    // Assert the "Connect Custom Software" heading is visible
    await expect(page.getByRole('heading', { name: 'Connect Custom Software' })).toBeVisible();
  });
});
