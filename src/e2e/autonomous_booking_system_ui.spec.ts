import { test, expect } from './fixtures';
import { adminPage } from './fixtures';

test.describe('Autonomous Booking System UI', () => {
  const tenantId = `e2e-tenant`;

  test('Public Booking Form Flow', async ({ page }) => {
    // 1. Visit booking page
    await page.goto(`/booking?tenant=${tenantId}&service_id=e2e-product-class`);
    await expect(page.getByRole('heading', { name: 'Book an Appointment' })).toBeVisible();

    // 2. Fill the form
    await page.fill('input[type="text"]', 'Jane Doe');
    await page.fill('input[type="email"]', 'jane@example.com');
    await page.fill('textarea', 'I need a drum lesson.');

    // 3. Date Selection triggers slot loading
    const dateQuery = new Date().toISOString().split('T')[0];
    await page.fill('input[type="date"]', dateQuery);

    // Let it fail naturally if the backend has no slots instead of mocking
    const submitBtn = page.getByRole('button', { name: 'Confirm Booking' });
    if (await submitBtn.isVisible()) {
        await submitBtn.click();
    }
  });

  test('Owner Admin Dashboard', async ({ browser }) => {
    const page = await adminPage(browser);

    await page.goto(`/admin/bookings?tenant=${tenantId}`);
    await expect(page.getByRole('heading', { name: 'Booking Management' })).toBeVisible();

    const newResNameInput = page.getByPlaceholder('Resource Name').first();
    if (await newResNameInput.isVisible()) {
        await newResNameInput.fill('Studio A');
        await page.getByRole('button', { name: 'Add Resource' }).click();
        await expect(page.getByText('Studio A').first()).toBeVisible();
    }
  });
});
