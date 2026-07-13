import { test, expect } from '../../../../e2e/fixtures';

test.describe('Autonomous Client Intake & Dynamic Quoting', () => {
  test('creates quote from intake and allows approval', async ({ request, page }) => {
    // We send a client intake request
    const formData = new URLSearchParams();
    formData.append('name', 'Carlos Handyman Test');
    formData.append('email', 'carlos-handyman@example.com');
    formData.append('details', 'Need my kitchen faucet replaced, here is a pic');

    const res = await request.post('/api/agents/client_intake?tenant=e2e-tenant', {
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded',
      },
      data: formData.toString()
    });

    expect(res.ok()).toBeTruthy();
    const result = await res.json();
    expect(result.success).toBe(true);
    expect(result.proposal_drafted).toBe(true);
    expect(result.quote_id).toBeDefined();

    const quoteId = result.quote_id;

    // Now Carlos navigates to the Quoting page
    await page.goto(`/quotes/${quoteId}`);

    // Expect the line item description for the service
    await expect(page.locator('text=x1')).toBeVisible({ timeout: 15000 });

    // Expect the price to be visible
    await expect(page.locator('text=$')).first().toBeVisible();

    // Find and click the Approve & Send Quote button
    const approveBtn = page.getByRole('button', { name: 'Approve & Send Quote' });
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // The quote should transition to SENT, wait for success (e.g. quote status to update or alert)
    // Actually the page doesn't show "SENT" natively if there is a stripe link but it might alert
    // wait for network or status update
    page.on('dialog', dialog => dialog.accept());

    await expect(page.locator('text=SENT')).toBeVisible({ timeout: 15000 });
  });
});
