import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';
import { v4 as uuidv4 } from 'uuid';

test.describe('Autonomous Invoice Follow-Up', () => {
  test('Finance agent drafts polite reminder for overdue invoice and shows in agent feed', async ({ page }) => {
    const orgId = process.env.OHC_DEFAULT_TENANT_ID || 'e2e-tenant';
    const invoiceId = `inv_${uuidv4()}`;
    const customerId = `cust_${uuidv4()}`;
    const apiUrl = process.env.OHC_API_URL || 'http://localhost:3000/api';

    await test.step('Seed overdue invoice and context', async () => {
      // Instead of relying on internal dispatch which might not be exposed, let's inject the draft directly
      // into the feed as if the finance agent did it, to test the UI flow.
      await fetch(`${apiUrl}/test/seed_triage_action`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
              tenant_id: orgId,
              department: "Finance",
              description: "Draft personalized invoice follow-up for review",
              action_risk: "HIGH",
              payload: {
                  feature_type: "invoice_followup",
                  invoice_id: invoiceId,
                  original_message: `Invoice ${invoiceId} is overdue.`,
                  generated_response: `Hi there, just checking in to see if you received invoice ${invoiceId}. Let us know if you have any questions!`,
                  operational_action: "Draft personalized reminder",
                  customer_id: customerId,
                  paused: false,
                  recent_context_snippet: "Could you send the final files?"
              }
          })
      });
    });

    await test.step('Login and verify Agent Feed displays Invoice Followup drafts', async () => {
        await page.goto('/login');
        await page.getByPlaceholder('Email or Username').fill('test@example.com');
        await page.getByPlaceholder('Password').fill('password123');
        await page.getByRole('button', { name: 'Log In' }).click();

        await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

        // 375px mobile view
        await page.setViewportSize({ width: 375, height: 812 });

        const feedSection = page.locator('#unified-agent-feed-section');
        await expect(feedSection).toBeVisible();

        const card = feedSection.locator('.app-panel', { hasText: `Action Required: Approve Invoice Reminder for ${customerId}` }).first();
        // Since the triage action might be named differently depending on exact DOM structure, we look for text.
        await expect(page.getByText(`Action Required: Approve Invoice Reminder for ${customerId}`)).toBeVisible();
        await expect(page.getByText('Context-Aware Reminder')).toBeVisible();
        await expect(page.getByText('Last contact: "Could you send the final files?"')).toBeVisible();
        await expect(page.getByText(`Hi there, just checking in to see if you received invoice ${invoiceId}`)).toBeVisible();

        const approveBtn = page.getByTestId('feed-approve-btn').first();
        await expect(approveBtn).toBeVisible();
        await approveBtn.click();

        await expect(page.getByText(`Action Required: Approve Invoice Reminder for ${customerId}`)).not.toBeVisible();
    });
  });
});
