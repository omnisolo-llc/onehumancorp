import { test, expect } from '@playwright/test';

test.describe('Interactive Walkthrough', () => {
  test('should load the walkthrough when test_walkthrough=true is present in the URL', async ({ page }) => {
    // Navigate to the Dashboard page with the test_walkthrough parameter
    await page.goto('/dashboard?test_walkthrough=true');

    // Wait for the first step of the walkthrough to be visible
    const firstStepTitle = page.locator('h3', { hasText: 'Set up your store' });
    await expect(firstStepTitle).toBeVisible();
    await expect(page.getByText('Step 1 of')).toBeVisible();

    // Click the "Next" button
    const nextButton = page.locator('button', { hasText: 'Next' });
    await expect(nextButton).toBeVisible();
    await nextButton.click();

    // Wait for the second step to be visible
    const secondStepTitle = page.locator('h3', { hasText: 'Accept your first payment' });
    await expect(secondStepTitle).toBeVisible();
    await expect(page.getByText('Step 2 of')).toBeVisible();

    // Click the "Next" button again
    await nextButton.click();

    // Let's verify step 3
    const thirdStepTitle = page.locator('h3', { hasText: 'Activate your AI Support Agent' });
    await expect(thirdStepTitle).toBeVisible();

    // Click the "Finish" button
    const finishButton = page.locator('button', { hasText: 'Finish' });
    await expect(finishButton).toBeVisible();
    await finishButton.click();

    // Verify the walkthrough has closed
    await expect(thirdStepTitle).not.toBeVisible();
  });
});
