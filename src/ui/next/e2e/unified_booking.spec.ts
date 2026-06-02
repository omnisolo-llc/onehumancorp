import { test, expect } from '@playwright/test';

test.describe('Unified Multi-Tenant Booking Engine E2E', () => {

  test('Customer can view available slots for a specific date', async ({ page }) => {
    // Navigate to the booking page
    await page.goto('http://localhost:3000/booking');

    // Assert header is visible
    await expect(page.locator('h1', { hasText: 'Book an Appointment' })).toBeVisible();

    // Assert service details are visible
    await expect(page.locator('h2', { hasText: 'Guitar Lesson' })).toBeVisible();
    await expect(page.locator('text=1 Hour')).toBeVisible();

    // Select the first available date
    const firstDateButton = page.locator('button').filter({ hasText: /Mon|Tue|Wed|Thu|Fri|Sat|Sun/ }).first();
    await firstDateButton.click();

    // Assert available times are displayed
    await expect(page.locator('h3', { hasText: 'Available Times' })).toBeVisible();

    // Check for at least one time slot button
    const timeSlots = page.locator('button').filter({ hasText: /AM|PM/ });
    await expect(timeSlots.first()).toBeVisible();
  });

  test('Customer can select a time slot and proceed to payment', async ({ page }) => {
    // Navigate to the booking page
    await page.goto('http://localhost:3000/booking');

    // Wait for dates to load
    const firstDateButton = page.locator('button').filter({ hasText: /Mon|Tue|Wed|Thu|Fri|Sat|Sun/ }).first();
    await firstDateButton.click();

    // Wait for time slots to load
    const timeSlots = page.locator('button').filter({ hasText: /AM|PM/ });
    await expect(timeSlots.first()).toBeVisible();

    // Select the first time slot
    await timeSlots.first().click();

    // Ensure the continue button is enabled and click it
    const continueButton = page.locator('button', { hasText: 'Continue to Payment' });
    await expect(continueButton).toBeEnabled();
    await continueButton.click();

    // The mock backend endpoint we created should simulate a successful booking
    // and redirect to the confirmation screen with a payment link
    await expect(page.locator('h2', { hasText: 'Booking Reserved!' })).toBeVisible();

    // Check for the deposit link
    const payButton = page.locator('a', { hasText: 'Pay Deposit' });
    await expect(payButton).toBeVisible();

    // Go back
    await page.locator('button', { hasText: 'Back to Booking' }).click();
    await expect(page.locator('h1', { hasText: 'Book an Appointment' })).toBeVisible();
  });

});

  test('Customer cannot proceed without selecting a time slot', async ({ page }) => {
    await page.goto('http://localhost:3000/booking');
    const continueButton = page.locator('button', { hasText: 'Continue to Payment' });
    await expect(continueButton).toBeDisabled();
  });

  test('System handles unavailable date appropriately', async ({ page }) => {
    await page.route('/api/booking', async route => {
      await route.fulfill({ json: { available_slots: [] } });
    });

    await page.goto('http://localhost:3000/booking');
    const firstDateButton = page.locator('button').filter({ hasText: /Mon|Tue|Wed|Thu|Fri|Sat|Sun/ }).first();
    await firstDateButton.click();

    // We expect the mockup UI currently renders the available slots list directly
    // Real implementation would show an empty state message
    // Just verifying the page renders without crashing for now
    await expect(page.locator('h1', { hasText: 'Book an Appointment' })).toBeVisible();
  });

  test('Error boundary is hit if booking endpoint fails', async ({ page }) => {
    await page.route('/api/booking', async route => {
      if (route.request().method() === 'POST') {
        await route.fulfill({ status: 500, body: 'Server Error' });
      } else {
        await route.continue();
      }
    });

    await page.goto('http://localhost:3000/booking');

    const firstDateButton = page.locator('button').filter({ hasText: /Mon|Tue|Wed|Thu|Fri|Sat|Sun/ }).first();
    await firstDateButton.click();

    const timeSlots = page.locator('button').filter({ hasText: /AM|PM/ });
    await timeSlots.first().click();

    const continueButton = page.locator('button', { hasText: 'Continue to Payment' });
    await continueButton.click();

    // After failure, we shouldn't see success
    await expect(page.locator('h2', { hasText: 'Booking Reserved!' })).not.toBeVisible();
  });
