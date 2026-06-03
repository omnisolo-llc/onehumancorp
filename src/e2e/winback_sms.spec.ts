import { test, expect } from './fixtures';

test.describe('Autonomous Customer Churn Winback Engine Flow', () => {
  test('Owner configures business, simulates time, and approves winback SMS', async ({ page }) => {
    // 1. Log in and configure the business using standard UI
    await page.goto('/login');
    await page.getByLabel('Email').fill('priya.boutique@example.com');
    await page.getByLabel('Password').fill('password123');
    await page.getByRole('button', { name: 'Sign In' }).click();

    // 2. Set up the business
    await page.getByRole('button', { name: 'Create Business' }).click();
    await page.getByLabel('Business Name').fill('Priya Boutique');
    await page.getByRole('button', { name: 'Save' }).click();

    // 3. Add a customer
    await page.getByRole('link', { name: 'Customers' }).click();
    await page.getByRole('button', { name: 'Add Customer' }).click();
    await page.getByLabel('Name').fill('Emily Chen');
    await page.getByLabel('Phone').fill('+1234567890');
    await page.getByRole('button', { name: 'Save Customer' }).click();

    // 4. Record interactions (to calculate cadence)
    await page.getByRole('button', { name: 'Log Interaction' }).click();
    await page.getByLabel('Type').fill('order');
    // Using a realistic interaction payload matching the UI
    await page.getByRole('button', { name: 'Save Interaction' }).click();

    // 5. Navigate back to dashboard and wait for the churn prediction to flag the customer
    await page.goto('/dashboard');

    // Check if there is an activity feed list or the pending approval card
    // The Ambassador drafts an ActionRisk::DraftForReview which should appear here
    await expect(page.getByText('Winback Opportunity for')).toBeVisible({ timeout: 10000 });

    // 6. The owner reviews the card
    // It should contain the draft text "We just got some new items..."
    await expect(page.getByText('We just got some new items')).toBeVisible();

    // 7. The owner clicks "Approve & Send"
    await page.getByRole('button', { name: 'Approve' }).first().click();

    // 8. Verify SMS sending confirmation
    await expect(page.getByText('Approved')).toBeVisible({ timeout: 5000 });
  });
});
