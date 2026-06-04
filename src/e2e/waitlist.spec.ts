import { test, expect } from '@playwright/test';

test.describe('Waitlist Page E2E', () => {
  test('User can successfully join the waitlist', async ({ page }) => {
    // Navigate to the waitlist page
    await page.goto('/waitlist');

    // Ensure the page loaded correctly
    await expect(page.locator('h1').first()).toContainText('The AI platform for');

    // Fill in the email input
    const emailInput = page.locator('input[type="email"]');
    await expect(emailInput).toBeVisible();
    await emailInput.fill('test-waitlist@example.com');

    // Submit the form
    const submitButton = page.getByRole('button', { name: 'Join the Waitlist' });
    await expect(submitButton).toBeVisible();
    await submitButton.click();

    // Verify the success message appears
    const successHeader = page.getByText("You're on the list!");
    await expect(successHeader).toBeVisible();
  });
});
