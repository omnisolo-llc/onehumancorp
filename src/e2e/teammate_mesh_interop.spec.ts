import { test, expect } from './fixtures';

test.describe('Teammate Mesh Interoperability Report', () => {
  test('Test 1: Start from the home page after user login, verify that the Team Activity section is visible', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Team Activity' })).toBeVisible();
  });

  test('Test 2: Verify that the "Swarm Online" indicator is visible in the Team Activity section', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByText('Swarm Online')).toBeVisible();
  });

  test('Test 3: Verify that the new Mesh Interoperability Transport indicator is visible after enabling Advanced Developer Settings', async ({ page }) => {
    await page.goto('/dashboard');

    // The transport pill should be hidden by default
    await expect(page.locator('text=Transport:').first()).toBeHidden();

    // Click profile menu to reveal settings
    await page.getByRole('button', { name: 'AC' }).click();

    // Toggle Advanced Developer Settings
    await page.getByRole('button', { name: 'Toggle Advanced Developer Settings' }).click();

    // Now the pill should be visible
    await expect(page.locator('text=Transport:').first()).toBeVisible();
  });

  test('Test 4: Navigate to the /agents page to verify that the active agents mesh status is loaded without errors', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'Your AI Team' })).toBeVisible();
    await expect(page.getByText('Active')).first().toBeVisible();
  });

  test('Test 5: Navigate back to the /dashboard and verify the Mesh indicator retains its state', async ({ page }) => {
    await page.goto('/dashboard');

    // Enable settings to start
    await page.getByRole('button', { name: 'AC' }).click();
    await page.getByRole('button', { name: 'Toggle Advanced Developer Settings' }).click();

    // Navigate away and back
    await page.goto('/agents');
    await page.getByRole('link', { name: 'Dashboard' }).click();

    // State should be retained (saved in localStorage)
    await expect(page.getByRole('heading', { name: 'Team Activity' })).toBeVisible();
    await expect(page.locator('text=Transport:').first()).toBeVisible();
  });
});
