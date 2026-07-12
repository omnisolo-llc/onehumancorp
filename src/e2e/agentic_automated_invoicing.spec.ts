import { test, expect, adminPage } from './fixtures';

import { v4 as uuidv4 } from 'uuid';

test.describe('Agentic Automated Invoicing & Cash Flow Management', () => {
  test('Finance agent automatically drafts invoice on project milestone completion', async ({ adminUser, loginAs, page, request }) => {
    let adminPageInstance = await adminPage(page);

    // Hit the simulation API route from the browser to carry auth cookies
    await page.evaluate(async () => {
      await fetch('/api/agents/approvals/simulate-invoice-draft', { method: 'POST' });
    });

    // Navigate to the Unified Agent Feed
    await page.goto('/dashboard.html');

    // Wait for the specific triage card
    const invoiceCard = page.locator('div[data-testid="agent-feed-card"]', { hasText: 'Generated Invoice' });
    await expect(invoiceCard).toBeVisible({ timeout: 15000 });
    await expect(invoiceCard).toContainText('Website Redesign');
    await expect(invoiceCard).toContainText('Phase 1 Complete');
    await expect(invoiceCard).toContainText('2500');

    // Approve the invoice
    const approveBtn = invoiceCard.getByTestId('feed-approve-btn');
    await expect(approveBtn).toHaveText(/Approve & Send/i);
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Verify it disappears (action processed)
    await expect(invoiceCard).not.toBeVisible({ timeout: 15000 });
  });
});
