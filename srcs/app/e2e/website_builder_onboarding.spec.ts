import { test, expect } from '@playwright/test';

test.use({ baseURL: 'http://localhost:8000' });

test.describe('Website Builder Onboarding E2E', () => {
  test('User can navigate through the website builder wizard flow from dashboard to publish', async ({ page }) => {
    // Login flow
    await page.goto('/');

    const loginLink = page.locator('text=continue to Cloud Dashboard');
    await expect(loginLink).toBeVisible();
    await loginLink.click();

    await expect(page).toHaveURL(/\/login/);
    await page.getByLabel('Email').fill('admin@example.com');
    await page.getByLabel('Password').fill('admin');
    await page.click('text=Sign In');

    await expect(page).toHaveURL(/\/dashboard/);

    // Wait for and click 'Build My Website'
    const buildBtn = page.locator('text=Build My Website');
    await expect(buildBtn).toBeVisible();
    await buildBtn.click();

    // Step 0: Choose Template
    await expect(page.locator('text=Choose a Template')).toBeVisible();
    await page.click('text=Modern Retail');
    await page.click('text=Next');

    // Step 1: Brand Colors & Logo
    await expect(page.locator('text=Brand Colors & Logo')).toBeVisible();
    await page.click('text=Generate'); // Mock AI generation
    await expect(page.locator('text=Logo ready!')).toBeVisible();
    await page.click('text=Next');

    // Step 2: Add Offering
    await expect(page.locator('text=Add your first offering')).toBeVisible();
    await page.getByLabel('Name').fill('Signature Cake');
    await page.getByLabel('Price').fill('45');
    // Click the auto-awesome icon or use the field directly
    await page.getByLabel('Short Description').fill('A delicious signature cake.');
    await page.click('text=Next');

    // Step 3: Connect Domain
    await expect(page.locator('text=Connect a domain')).toBeVisible();
    await page.click('text=Use a free OHC subdomain');
    await page.click('text=Next');

    // Step 4: Ready to publish
    await expect(page.locator('text=Ready to go live!')).toBeVisible();
    await expect(page.locator('text=Live Preview')).toBeVisible();
    await page.click('text=Publish');

    // After publish it should go back to the dashboard or show success.
    // Wait for the simulated delay (2 seconds) and redirection.
    await expect(page).toHaveURL(/\/dashboard/);
  });
});
