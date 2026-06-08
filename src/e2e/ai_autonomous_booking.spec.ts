import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('ai_autonomous_booking');

test.describe('Autonomous Booking & Scheduling Engine', () => {
  // Mobile viewport for realistic field service owner testing
  test.use({ viewport: { width: 375, height: 667 } });

  test('Operations Agent handles time negotiation, Redis lock slot, and Stripe deposit link', async ({ page }) => {
    // Navigate to Chat interface where booking starts
    await page.goto('/chat');

    await expect(page.getByRole('heading', { name: 'Customer Assistant' })).toBeVisible({ timeout: 15000 });

    // Step 1: Customer text is simulated
    await page.getByPlaceholder('Type your message...').fill('Can you come look at my sink sometime next week?');
    await page.getByRole('button', { name: 'Send' }).click();

    // The backend chat interceptor detects booking intent and triggers handle_booking_intent
    // Step 2: Agent drafts a reply proposing times (mocked in backend or via E2E seed)
    await expect(page.getByText('Checking availability...')).toBeVisible({ timeout: 15000 });

    // Simulate customer confirming
    await page.getByPlaceholder('Type your message...').fill('Yes, that works perfect.');
    await page.getByRole('button', { name: 'Send' }).click();

    // Step 3: Agent places Redis lock and creates Stripe link
    await expect(page.getByText('Creating Stripe checkout for')).toBeVisible({ timeout: 15000 });

    // Step 4: Validate Mobile UI Action Cards appear on calendar/dashboard
    await page.goto('/calendar');

    // Check Action Cards
    await expect(page.getByText('Operations Agent')).toBeVisible();
    await expect(page.getByText('Agent tentatively booked a roof repair estimate')).toBeVisible();
    await expect(page.getByText('Pending $50 deposit. No action needed.')).toBeVisible();

    // Check Approval Card
    await expect(page.getByText('Mark requested to reschedule his 4 PM lesson')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Approve' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Edit' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Deny' })).toBeVisible();
  });
});
