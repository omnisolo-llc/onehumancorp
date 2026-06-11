import { test, expect } from '@playwright/test';

test.describe('Agentic Omnichannel Returns', () => {
  test('should display return requests and allow approval', async ({ page, request }) => {
    // Navigate to the dashboard first to set tenant and any needed state
    await page.goto('http://localhost:3000/dashboard');

    // Seed the backend with a return request to ensure we have data
    const seedRes = await request.post('http://localhost:3000/api/v1/returns/requests/seed');
    expect(seedRes.ok()).toBeTruthy();
    const seedData = await seedRes.json();
    const orderId = seedData.id;

    // Navigate to the returns page
    await page.goto('http://localhost:3000/returns');

    // Wait for the page to load
    await expect(page.locator('text=Returns & Exchanges')).toBeVisible();

    // Verify the return request card is displayed using the seeded order ID
    const returnCard = page.locator(`text=Return requested by Sarah for Order #${orderId}.`);
    await expect(returnCard).toBeVisible();

    // Verify the operations agent message
    const opsMessage = page.locator('text=Operations Agent has generated a return label and prepared a $45.00 refund.');
    await expect(opsMessage).toBeVisible();

    // Click the Approve button
    const approveButton = page.locator('button', { hasText: 'Approve' }).first();
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // Verify the success message
    const successMessage = page.locator('text=Approved! Return label generated and refund processed.');
    await expect(successMessage).toBeVisible();

    // Now if it was the only one, we might see the empty message
    // Let's verify that the approved card is no longer visible
    await expect(returnCard).not.toBeVisible();
  });
});
