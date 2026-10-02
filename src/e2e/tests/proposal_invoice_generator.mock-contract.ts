import { test, expect } from '../fixtures';

test.describe('Agentic Proposal & Invoice Generator for Service Agencies', () => {
  test('draft proposal, approve, and verify auto-generated invoice', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.setViewportSize({ width: 375, height: 812 });

    // 1. Simulate the backend having autonomously drafted a proposal for an inquiry.
    // We mock the backend creating it by calling the draft_agent endpoint directly in test
    const response = await page.request.post(`/api/v1/proposals/draft_agent`, {
      data: {
        inquiry: "Can you design a new logo for ACME Corp?",
        customer_id: "test-customer-acme-123",
        tenant_id: "e2e-tenant",
      }
    });

    expect(response.ok()).toBeTruthy();
    const { id: proposalId } = await response.json();
    expect(proposalId).toBeDefined();

    // 2. Nora opens OmniSolo app and navigates to review AI-generated proposal
    await page.goto(`/proposals/${proposalId}`);
    await page.waitForLoadState('networkidle');

    // 3. Review UI
    await expect(page.locator('text=Review Proposal')).toBeVisible();
    await expect(page.locator('text=AI Proposal Design')).toBeVisible(); // from mocked LLM response
    await expect(page.locator('text=$250.00')).toBeVisible(); // $250.00 from 25000 cents

    // 4. Tap "Approve & Send"
    const approveButton = page.locator('button:has-text("Approve & Send")');
    await expect(approveButton).toBeVisible();

    page.on('dialog', dialog => dialog.accept());
    await approveButton.click();

    // 5. Verify UI shows ACCEPTED and Stripe payment link
    await expect(page.locator('text=ACCEPTED')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Stripe Payment Link')).toBeVisible();

    // 6. Navigate to /finance and verify invoice is auto-generated
    await page.goto(`/finance`);
    await page.waitForLoadState('networkidle');

    await expect(page.locator('h1', { hasText: 'Finance & Invoicing' })).toBeVisible();

    // We should see an invoice with the $250.00 amount or the customer name
    // The invoice table might list amounts
    await expect(page.locator('text=$250.00').first()).toBeVisible();
  });
});
