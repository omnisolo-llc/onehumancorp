import { test, expect } from '@playwright/test';

test.describe('Funding Opportunities', () => {
  test('should display funding opportunities and allow approval', async ({ page }) => {
    // Navigate to the dashboard or activity feed where the funding card would be shown
    await page.goto('/');

    // Wait for the funding opportunity card to appear
    const fundingCard = page.locator('text="Downtown Revitalization Grant"');
    await expect(fundingCard).toBeVisible();

    // Click the review proposal button (assuming there's a button or the card itself is clickable)
    const reviewButton = fundingCard.locator('button:has-text("Review")');
    if (await reviewButton.isVisible()) {
        await reviewButton.click();
    } else {
        await fundingCard.click();
    }

    // Modal with the AI proposal should appear
    const modalText = page.locator('text="AI-generated proposal"');
    await expect(modalText).toBeVisible();

    // Find and click the Submit Application button
    const submitButton = page.locator('button:has-text("Submit Application")');
    await expect(submitButton).toBeVisible();
    await submitButton.click();

    // Assert that it was submitted (e.g., success message or card status update)
    const submittedText = page.locator('text="Submitted"');
    await expect(submittedText).toBeVisible();
  });
});
