import { test, expect } from '@playwright/test';

test.describe('Onboarding UI Audit', () => {
  test('Should navigate the setup successfully via Conversational Setup', async ({ page }) => {
    // Start at the onboarding page
    await page.goto('http://localhost:3000/onboarding');

    // Check initial state title
    await expect(page.locator('h1').filter({ hasText: 'Setup' })).toBeVisible();

    // Step -2: Welcome Screen
    await expect(page.getByText('10-Minute Setup Wizard')).toBeVisible();

    // "Start My Business" navigates to `step: 1` which is "Tell us about your business" in the conversational setup.
    await page.getByRole('button', { name: 'Start My Business' }).click();

    await expect(page.getByText('What\'s the name of your business?')).toBeVisible({ timeout: 10000 });

    await page.getByPlaceholder('e.g. Maya\'s Custom Cakes').fill('My Test Store');
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    // Enter what you sell
    await expect(page.getByText('What do you sell?')).toBeVisible();
    await page.locator('textarea').fill('Cookies');
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    // Location
    await expect(page.getByText('Where are you located?')).toBeVisible();
    await page.getByPlaceholder('e.g. Portland, OR').fill('New York');
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    // Target Audience
    await expect(page.getByText('Who is your target audience?')).toBeVisible();
    await page.getByPlaceholder('e.g. Local families, Tech startups').fill('Everyone');
    await page.getByRole('button', { name: 'Next', exact: true }).click();

    // Wait for the processing to finish by looking for the Review Details screen
    await expect(page.getByText('Review Details')).toBeVisible({ timeout: 30000 });

    // Advance to Style
    await page.getByRole('button', { name: 'Continue' }).click();

    // Fill in Account Setup fields
    await expect(page.getByText('Style & Team')).toBeVisible();
    await page.getByPlaceholder('e.g. Maya Smith').fill('Test User');
    await page.getByPlaceholder('you@example.com').fill('testuser@example.com');
    await page.getByPlaceholder('••••••••').fill('password123');

    // Submit
    await page.getByRole('button', { name: 'Approve & Publish' }).click();

    // Verify Success Screen
    await expect(page.getByText('You\'re Live!')).toBeVisible({ timeout: 20000 });
  });
});
