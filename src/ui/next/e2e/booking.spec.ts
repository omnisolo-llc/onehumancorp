import { test, expect } from '@playwright/test';

test.describe('Autonomous Booking Agent Flow', () => {
  // Mock localStorage to simulate a logged-in user
  test.use({
    storageState: {
      cookies: [],
      origins: [
        {
          origin: 'http://localhost:3000',
          localStorage: [
            { name: 'has_onboarded', value: 'true' },
            { name: 'tenant', value: 'test-tenant' }
          ]
        }
      ]
    }
  });

  test('completes full booking flow from dashboard to payment confirmation', async ({ page }) => {
    // 1. Start from the home page after user login
    await page.goto('http://localhost:3000/');

    // Because of the mock localStorage, it should redirect to dashboard
    await page.waitForURL('**/dashboard');
    await expect(page.locator('h1:has-text("Dashboard")')).toBeVisible();

    // 2. Navigate the entire feature flow
    // Click the new "Simulate Booking" link
    await page.click('#simulate-booking-link');

    // Wait for the booking page to load
    await page.waitForURL('**/booking');

    // 3. Check for the AI greeting
    await expect(page.locator('text="Carlos\' Assistant"')).toBeVisible();
    await expect(page.locator('text="Hi there! 👋 I am the AI assistant for Carlos Handyworks. How can we help you today?"')).toBeVisible();

    // 4. User types a message (Qualification phase)
    const inputField = page.locator('input[placeholder="Type your message..."]');
    await inputField.fill('I have a leaky pipe that needs fixing.');
    await inputField.press('Enter');

    // User message should appear
    await expect(page.locator('text="I have a leaky pipe that needs fixing."')).toBeVisible();

    // Wait for AI to respond with time slots
    await expect(page.locator('text="Thanks for letting me know. I can definitely help schedule a time for Carlos to come take a look at that. Here are a few available times based on his calendar:"')).toBeVisible({ timeout: 5000 });

    // 5. Select a time slot
    await page.click('button:has-text("Tomorrow at 10:00 AM")');

    // User message for slot selection should appear
    await expect(page.locator('div', { hasText: 'Tomorrow at 10:00 AM' }).nth(2)).toBeVisible(); // or just wait for confirmation

    // 6. Assert confirmation and payment link appear
    await expect(page.locator('text="Great! I\'ve provisionally booked Carlos for Tomorrow at 10:00 AM. To confirm the appointment, please click below to submit the $50 deposit."')).toBeVisible({ timeout: 5000 });

    const paymentButton = page.locator('#pay-deposit-btn');
    await expect(paymentButton).toBeVisible();
    await expect(paymentButton).toHaveText('Pay $50 Deposit');
  });
});
