import { test, expect } from '@playwright/test';

// fabricated business payload
// synthetic response
// network interception

test.describe('Unified Multi-Tenant Calendar & Booking Engine', () => {
    test('Customer asks for time in DM -> Agent answers with slot -> Booking confirmed in UI', async ({ page, request }) => {
        // Log in as an admin or tenant owner
        await page.goto('/login');
        await page.fill('input[name="email"]', 'admin@onehumancorp.com');
        await page.fill('input[name="password"]', 'password123');
        await page.click('button[type="submit"]');

        await expect(page).toHaveURL('/dashboard');

        // Go to settings/services and ensure a service and calendar are setup
        await page.click('text=Services');

        await page.click('text=New Service');
        await page.fill('input[name="title"]', 'Plumbing Consultation');
        await page.fill('input[name="price"]', '100');
        await page.click('button[type="submit"]');

        // Go to Inbox
        await page.click('text=Inbox');
        await expect(page.locator('h1')).toContainText('Inbox');

        // Send a message as a customer to the agent
        await page.click('text=New Conversation');
        await page.fill('input[placeholder="Customer Name"]', 'Test Customer');
        await page.fill('textarea[placeholder="Type message..."]', 'I need a Plumbing Consultation tomorrow.');
        await page.click('button:has-text("Send")');

        // The operations agent should respond with an available slot (Draft Booking).
        // Wait for agent reply proposing a time
        await expect(page.locator('.message.agent')).toContainText('tomorrow');

        // The customer confirms the booking
        await page.fill('textarea[placeholder="Type message..."]', 'Yes, that works for me.');
        await page.click('button:has-text("Send")');

        // Check the calendar view to ensure the booking was confirmed
        await page.click('text=Calendar');
        await expect(page.locator('.booking-event')).toContainText('Plumbing Consultation');
    });
});