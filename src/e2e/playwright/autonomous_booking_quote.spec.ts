import { test, expect } from '@playwright/test';

test.describe('Autonomous Booking & Quoting E2E', () => {
    test('Carlos creates a service and views AI Operations Agent triage', async ({ page }) => {
        // Go to dashboard
        await page.goto('/ui/dashboard.html');
        await expect(page.locator('text=Welcome back')).toBeVisible();

        // 1. Carlos clicks "Create Service"
        const createServiceLink = page.locator('a[href="booking-create.html"]');
        await expect(createServiceLink).toBeVisible();

        // Pass a tenant for backend auth in UI calls
        await page.goto('/ui/booking-create.html?tenant=carlos-handyman');

        await expect(page.locator('text=Create a Service')).toBeVisible();

        // Fill out the service form
        await page.fill('#title', 'Sink Repair');
        await page.fill('#price', '50');
        await page.fill('#description', 'Fix leaky sinks and replace pipes.');

        // Wait for potential setup before submitting to let JS initialize
        await page.waitForTimeout(500);

        // Submit the form
        await page.click('button[type="submit"]');

        // Wait for success screen
        await expect(page.locator('text=Service Created!')).toBeVisible({ timeout: 5000 });

        // Go back to the dashboard
        await page.click('button:has-text("Go to Dashboard")');
        await expect(page.locator('text=Bookings Dashboard')).toBeVisible();

        // 2. Carlos sees the AI Operations Agent card
        const aiAgentCard = page.locator('text=Operations Agent');
        await expect(aiAgentCard).toBeVisible();

        const aiAgentDesc = page.locator('text=Drafted 3 booking replies and scheduled 2 visits for tomorrow.');
        await expect(aiAgentDesc).toBeVisible();

        // Verify action buttons exist
        const approveBtn = page.locator('button:has-text("Approve & Send Link")');
        await expect(approveBtn).toBeVisible();

        const editBtn = page.locator('button:has-text("Edit")');
        await expect(editBtn).toBeVisible();
    });
});
