import { test, expect } from '@playwright/test';

test.describe('Agentic Service Booking & Quoting CUJ', () => {
  test('Owner configures settings then Customer requests a service and Owner approves AI quote draft', async ({ page }) => {
    // 0. Owner configures settings
    await page.goto('http://localhost:3000/login');
    await page.getByPlaceholder('Email or Username').fill('carlos@ohc.test');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Login' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    await page.goto('http://localhost:3000/team');
    await expect(page.getByRole('heading', { name: 'Your Team', exact: true })).toBeVisible();

    await page.waitForTimeout(2000);
    await page.getByRole('button', { name: 'The Salesperson' }).first().click();
    await expect(page.getByRole('heading', { name: 'The Salesperson' })).toBeVisible({ timeout: 5000 });

    // Ensure the toggle is OFF
    const toggleBtn = page.getByRole('button').filter({ hasText: '' }).first();
    const isChecked = await toggleBtn.evaluate((node) => node.className.includes('bg-blue-500'));
    if (isChecked) {
      await toggleBtn.click();
      await page.waitForTimeout(1000);
    }

    // 1. Customer Flow
    // Navigate to booking form
    await page.goto('http://localhost:3000/booking');

    // Check elements
    await expect(page.getByRole('heading', { name: 'Request a Service' })).toBeVisible();

    // Fill form
    await page.getByPlaceholder('e.g. I have a leaky faucet in the kitchen that needs fixing.').fill('I need help fixing a leaky pipe in my kitchen sink.');

    // Submit form
    await page.getByRole('button', { name: 'Get a Quote' }).click();

    // Verify submission success
    await expect(page.getByRole('heading', { name: 'Request Sent!' })).toBeVisible();
    await expect(page.getByText("We've received your inquiry.")).toBeVisible();


    // 2. Owner Flow
    // Navigate back to Team
    await page.goto('http://localhost:3000/team');
    await expect(page.getByRole('heading', { name: 'Your Team', exact: true })).toBeVisible();

    // Wait for the modal or card to fully render
    await page.waitForTimeout(5000);

    // Click on Salesperson department
    await page.getByRole('button', { name: 'The Salesperson' }).first().click();

    // Wait for the modal or card to fully render
    await page.waitForTimeout(5000);

    // Wait for network requests to finish
    await page.waitForLoadState('networkidle');

    // Ensure we are viewing the Salesperson inbox specifically
    await expect(page.getByRole('heading', { name: 'The Salesperson' })).toBeVisible({ timeout: 5000 });

    // Ensure data is loaded
    await expect(page.getByText('Autonomous Quoting')).toBeVisible({ timeout: 15000 });

    // Wait for the specific inquiry text to appear, indicating the quote card is loaded
    const inquiryLocator = page.getByText('I need help fixing a leaky pipe in my kitchen sink.').first();

    // We expect it to be visible since autonomous quoting is disabled by default
    await expect(inquiryLocator).toBeVisible({ timeout: 10000 });

    // Verify the rest of the quote draft card
    await expect(page.getByText('Quote generated for review').first()).toBeVisible();

    // Click Approve
    await page.getByRole('button', { name: 'Approve' }).first().click();

    // Validate empty state or removal
    await expect(page.getByText('Quote generated for review')).toBeHidden();
  });

  test('Owner configures Autonomous Quoting and Service is Auto-Approved', async ({ page }) => {
    // 1. Owner Flow - Configure Settings
    await page.goto('http://localhost:3000/login');
    await page.getByPlaceholder('Email or Username').fill('carlos@ohc.test');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Login' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    await page.goto('http://localhost:3000/team');
    await expect(page.getByRole('heading', { name: 'Your Team', exact: true })).toBeVisible();

    await page.waitForTimeout(2000);
    await page.getByRole('button', { name: 'The Salesperson' }).first().click();

    await expect(page.getByRole('heading', { name: 'The Salesperson' })).toBeVisible({ timeout: 5000 });
    await expect(page.getByText('Autonomous Quoting')).toBeVisible({ timeout: 5000 });

    // Toggle Autonomous Quoting to ON
    const toggleBtn = page.getByRole('button').filter({ hasText: '' }).first();
    await toggleBtn.click();

    // Enter pricing rules
    await page.getByPlaceholder('e.g. $50/hr base, plus materials').fill('$50/hr base, plus materials');

    // Click outside to trigger save
    await page.getByRole('heading', { name: 'The Salesperson' }).click();
    await page.waitForTimeout(1000); // give it a moment to save

    // 2. Customer Flow
    await page.goto('http://localhost:3000/booking');
    await expect(page.getByRole('heading', { name: 'Request a Service' })).toBeVisible();
    await page.getByPlaceholder('e.g. I have a leaky faucet in the kitchen that needs fixing.').fill('My roof is leaking and I need it patched immediately.');
    await page.getByRole('button', { name: 'Get a Quote' }).click();

    await expect(page.getByRole('heading', { name: 'Request Sent!' })).toBeVisible();
    await expect(page.getByText("We've received your inquiry.")).toBeVisible();

    // 3. Verify it was auto-approved
    await page.goto('http://localhost:3000/team');
    await page.waitForTimeout(2000);
    await page.getByRole('button', { name: 'The Salesperson' }).first().click();

    await expect(page.getByRole('heading', { name: 'The Salesperson' })).toBeVisible({ timeout: 5000 });

    // Ensure the new inquiry is NOT in the pending approval list
    await expect(page.getByText('My roof is leaking and I need it patched immediately.')).toBeHidden();
  });
});
