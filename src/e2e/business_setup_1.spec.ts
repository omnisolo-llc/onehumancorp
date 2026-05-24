import { test, expect } from './fixtures';

test.describe('Business Setup Wizard', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.locator('#setup-screen')).toBeVisible();
  });

  test('shows the current setup welcome step', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Your business, live in minutes.' })).toBeVisible();
    await expect(page.getByRole('button', { name: /Start My Business/ })).toBeVisible();
    await expect(page.getByRole('button', { name: /Instant Build/ })).toBeVisible();
  });

  test('moves through business type and name steps', async ({ page }) => {
    await page.getByRole('button', { name: /Start My Business/ }).click();
    await expect(page.getByRole('heading', { name: 'What kind of business are you building?' })).toBeVisible();

    await page.getByRole('button', { name: /Online Store/ }).click();
    await expect(page.getByRole('heading', { name: 'Give your business a name' })).toBeVisible();
    await page.getByPlaceholder('What is your business called?').fill('Test Company');
    await page.getByRole('button', { name: /Next/ }).click();
    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
  });

  test('completes the publish path to the checklist', async ({ page }) => {
    await page.getByRole('button', { name: /Start My Business/ }).click();
    await page.getByRole('button', { name: /Online Store/ }).click();
    await page.getByPlaceholder('What is your business called?').fill('Test Company');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByLabel(/Physical Products/).check();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByPlaceholder('What is the name of this product?').fill('Custom Cookies');
    await page.getByPlaceholder('0.00').fill('24.99');
    await page.getByRole('button', { name: /Next/ }).click();
    await expect(page.getByRole('heading', { name: 'How do you want to receive payments?' })).toBeVisible();
    await page.getByRole('button', { name: 'Online', exact: true }).click();
    await page.getByPlaceholder('e.g. Maya Smith').fill('Maya Smith');
    await page.getByPlaceholder('you@email.com').fill('maya@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: 'Modern' }).click();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Free OHC Domain/ }).click();
    await page.getByRole('button', { name: /Next/ }).click();
    await page.getByRole('button', { name: /Publish my business/ }).click();

    await expect(page.getByRole('heading', { name: /Success! Your business is live!/ })).toBeVisible();
    await page.getByRole('button', { name: /View Welcome Checklist/ }).click();
    await expect(page.getByText("You're set up! Here's what to do next:")).toBeVisible();
  });
});
