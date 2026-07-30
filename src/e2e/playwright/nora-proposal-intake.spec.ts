import { test, expect } from '@playwright/test';

test.describe('Nora Autonomous Proposal Intake Flow', () => {
  let proposalId: string;
  let tenantId = 'agency-1';
  let customerId = 'cust-1';

  test('Client intake creates proposal automatically', async ({ request, page }) => {
    // Simulate Client Inquiry

    const body = await res.json();
    proposalId = body.proposal.id;
    expect(proposalId).toBeDefined();
    expect(body.proposal.project_scope).toBe("Website Redesign & Branding");

    // Check Client View
    await page.goto(`/proposals/customer-view?id=${proposalId}`);
    // Assume we'd verify client view here.
  });
});
