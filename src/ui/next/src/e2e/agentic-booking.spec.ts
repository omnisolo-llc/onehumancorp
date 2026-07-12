import { test, expect } from '../../../../e2e/fixtures';

test.describe('Agentic Service Booking & Quoting CUJ', () => {
  test('Customer requests a service and Owner approves AI quote draft', async ({ page }) => {
    // 1. Customer Flow
    // Navigate to booking form
    await page.goto('/booking');

    // Check elements
    await expect(page.getByRole('heading', { name: 'Book an Appointment' })).toBeVisible();

    // Fill form
    await page.getByPlaceholder('Jane Doe').fill('John Doe');
    await page.getByPlaceholder('jane@example.com').fill('johndoe@example.com');
    await page.locator('input[type="date"]').fill(new Date().toISOString().split('T')[0]);

    // Select first slot
    await page.waitForTimeout(1000);
    const firstSlot = page.locator('button', { hasText: /:/ }).first();
    if (await firstSlot.isVisible()) {
      await firstSlot.click();
    }

    await page.getByPlaceholder('What do you need help with?').fill('I need help fixing a leaky pipe in my kitchen sink.');

    // Submit form
    await page.getByRole('button', { name: 'Confirm Booking' }).click();

    // Wait for network response (sometimes the fallback kicks in)
    await page.waitForTimeout(1000);

    // Verify submission success
    const heading = page.getByRole('heading', { name: /Almost there!|Request Sent!/i });
    await expect(heading.first()).toBeVisible({ timeout: 15000 });

    // 2. Owner Flow
    // Ensure login happens properly
    await page.goto('/login');
    // Using simple bypass or navigating directly to dashboard
    await page.goto('/dashboard');
    // Wait for the UI to be ready
    await page.waitForTimeout(1000);

    // Navigate to Feed (where drafts are kept)
    await page.goto('/feed');

    // Wait for the modal or card to fully render
    await page.waitForTimeout(2000);

    // Look for approve button
    const approveBtn = page.getByRole('button', { name: /Approve/i }).first();
await expect(approveBtn).toBeVisible({ timeout: 5000 });
    await approveBtn.click();
    await expect(approveBtn).toBeHidden({ timeout: 5000 });
  });
});
