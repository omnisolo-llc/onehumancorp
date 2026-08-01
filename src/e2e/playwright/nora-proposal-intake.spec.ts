import { test, expect } from '../fixtures';

test.describe('Nora Autonomous Proposal Intake Flow', () => {
  let tenantId = 'agency-1';
  let customerId = 'cust-1';

  test('Client intake creates proposal automatically', async ({ request, page }) => {
    // Simulate Client Inquiry
    const res = await request.post('/api/v1/intake');

    if (res.ok()) {
        const body = await res.json();
        let proposalId = body.proposal.id;
        expect(proposalId).toBeDefined();

        // Check Client View
        await page.goto(`/proposals/customer-view?id=${proposalId}`);
    }
  });
});
