import { test, expect } from '@playwright/test';
import { e2eConfig, setupTenantAndUser } from '../playwright.config';

test.describe('Agentic Proposal & Invoice Generator for Service Agencies', () => {
  let context: any;

  test.beforeEach(async ({ browser }) => {
    context = await setupTenantAndUser(browser);
  });

  test('draft proposal, approve, and verify auto-generated invoice', async () => {
    const page = await context.newPage();
    await page.setViewportSize({ width: 375, height: 812 });

    // 1. Simulate the backend having autonomously drafted a proposal for an inquiry.
    // We mock the backend creating it by calling the draft_agent endpoint directly in test
    const response = await page.request.post(`${e2eConfig.baseURL}/api/proposals/draft_agent`, {
      data: {
        inquiry: "Can you design a new logo for ACME Corp?",
        customer_id: "test-customer-acme-123",
        tenant_id: "e2e-tenant",
      }
    });

    expect(response.ok()).toBeTruthy();
    const { id: proposalId } = await response.json();
    expect(proposalId).toBeDefined();

    // 2. Nora opens OHC app and navigates to review AI-generated proposal
    await page.goto(`${e2eConfig.baseURL}/proposals/${proposalId}`);
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
    await page.goto(`${e2eConfig.baseURL}/finance`);
    await page.waitForLoadState('networkidle');

    await expect(page.locator('h1', { hasText: 'Finance & Invoicing' })).toBeVisible();

    // We should see an invoice with the $250.00 amount or the customer name
    // The invoice table might list amounts
    await expect(page.locator('text=$250.00').first()).toBeVisible();
  });
});
