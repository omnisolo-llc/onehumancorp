import { test, expect } from '@playwright/test';

test.describe('Day One Onboarding', () => {
  test('User can register and complete the business setup wizard', async ({ page }) => {
    // Navigate to the app
    await page.goto('/');

    // Ensure we are on the login page and click "Sign Up"
    await page.getByRole('button', { name: 'Don\'t have an account? Sign Up' }).click();

    // Fill in registration details
    await page.getByLabel('Email').fill('test_ceo@example.com');
    await page.getByLabel('Username').fill('test_ceo');
    await page.getByLabel('Password').fill('dummy_password');

    // Submit registration
    await page.getByRole('button', { name: 'Sign Up' }).click();

    // Verification screen
    await expect(page.getByText('Check your email')).toBeVisible();
    await page.getByRole('button', { name: 'I have verified my email' }).click();

    // We should now be in the Business Setup Wizard Step 0
    await expect(page.getByText('Welcome! Your AI team, ready in minutes.')).toBeVisible();
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 1: Business Type & Name
    await page.getByLabel('Business Type (e.g. Baker, Handyman)').fill('Baker');
    await page.getByLabel('Company Name').fill('Maya Cakes');
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 2: What do you sell & Payment
    await page.getByLabel('What do you sell?').fill('Custom cakes');
    // Default payment is stripe, keep it
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 3: Template Selection
    await expect(page.getByText('Choose your style')).toBeVisible();
    await page.getByText('Modern').click(); // Select Modern template
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 4: First Product
    await expect(page.getByText('Add your first product/service')).toBeVisible();
    await page.getByLabel('Product Name').fill('Vegan Chocolate Cake');
    await page.getByLabel('Description (✨ AI will expand this)').fill('Delicious vegan chocolate cake');
    await page.getByLabel('Price').fill('45.00');
    // Mock upload photo click
    await page.getByRole('button', { name: 'Upload Photo' }).click();
    await expect(page.getByText('Photo uploaded and cropped!')).toBeVisible();
    await page.getByRole('button', { name: 'Next' }).click();

    // Step 5: Domain Selection
    await expect(page.getByText('Claim your domain')).toBeVisible();
    await page.getByLabel('Subdomain').fill('mayacakes');
    await expect(page.getByText('https://mayacakes.ohc.app')).toBeVisible();

    // Go Live
    await page.getByRole('button', { name: 'Publish 🎉' }).click();

    // Assert Confetti and Clipboard SnackBar
    await expect(page.getByText('🎉 Published! Link copied to clipboard:')).toBeVisible();

    // Welcome Checklist
    await expect(page.getByText('You\'re set up! Here\'s what to do next:')).toBeVisible();
    await expect(page.getByText('Business live')).toBeVisible();
  });
});
