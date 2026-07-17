import { test, expect } from '../../../../e2e/fixtures';

test.describe('Offline-Tolerant Proposal to Invoice CUJ', () => {
  test('Owner reviews a draft proposal, modifies it, and sends it offline', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);

    // 1. Setup a draft proposal by calling the API
    const createProposalRes = await page.request.post('/api/v1/proposals', {
      headers: {
        'x-tenant-id': 'tenant-1'
      },
      data: {
        customer_id: 'customer-1',
        total_amount: 15000,
        line_items: [
          {
            description: 'Drywall Repair',
            unit_price_cents: 15000,
            quantity: 1,
            is_optional: false
          }
        ]
      }
    });

    const proposalData = await createProposalRes.json();
    const proposalId = proposalData.id;

    // Navigate to the quoting page for this ID
    await page.goto(`/quoting?id=${proposalId}`);

    // 2. Wait for the page to load
    await expect(page.getByText('Proposal Summary')).toBeVisible();
    await expect(page.getByText('Drywall Repair')).toBeVisible();

    // 3. Edit the quantity of the line item
    const quantityInput = page.locator('input[type="number"]').first();
    await quantityInput.fill('2');

    // 4. Verify total updates
    // Drywall Repair: 150.00 * 2 = 300.00
    await expect(page.getByTestId('proposal-total')).toHaveText('$300.00');

    // 5. Go offline using CDP to simulate offline environment
    const context = page.context();
    await context.setOffline(true);
    await expect(page.getByText("Working offline. Changes saved.")).toBeVisible();

    // 6. Click Approve & Send
    const approveBtn = page.getByTestId('proposal-approve-btn');
    await approveBtn.click();

    // 7. Verify optimistic UI update (Proposal Accepted)
    await expect(page.getByText('Proposal Accepted')).toBeVisible();
    await expect(page.getByText('Thank you! This proposal has been approved.')).toBeVisible();

    // 8. Restore connection and wait for sync
    await context.setOffline(false);

    // Wait for the sync to complete
    await page.waitForTimeout(2000);

    // 9. Verify the backend status via API
    const getProposalRes = await page.request.get(`/api/v1/proposals/${proposalId}`, {
      headers: {
        'x-tenant-id': 'tenant-1'
      }
    });

    const updatedProposalData = await getProposalRes.json();
    expect(updatedProposalData.proposal.status).toBe('ACCEPTED');
    expect(updatedProposalData.proposal.total_amount).toBe(30000);

    // Check line items got updated
    const lineItem = updatedProposalData.line_items.find((i: any) => i.description === 'Drywall Repair');
    expect(lineItem.quantity).toBe(2);
  });
});
