import { test, expect } from './fixtures';

test.describe('Nora Intake Proposal Flow (375px viewport)', () => {
  // Mobile first viewport
  test.use({ viewport: { width: 375, height: 667 } });

  test('generates and displays proposal draft on agent feed', async ({ page, request, loginAs, adminUser }) => {
    await loginAs(page, adminUser);

    // Simulate SalesAgent creating an approval in DB since work-intake/submit endpoint triggers a webhook which requires the real rust backend
    await request.post('/api/agents/approvals/simulate-quote-draft', {
      headers: {
        'x-tenant-id': 'tenant-1',
        'x-user-id': 'default'
      },
      data: {
        inbox_message_id: 'msg-1'
      }
    });

    await page.goto('/dashboard');

    // Wait for the "quote_draft" card to appear in the dashboard.
    await expect(async () => {
      await page.waitForTimeout(1000); await page.reload();
      const quoteDraftCard = page.getByTestId('quote-draft-card').first();
      await expect(quoteDraftCard).toBeVisible({ timeout: 5000 });
    }).toPass({
      intervals: [2000, 5000, 10000],
      timeout: 30000,
    });

    // Check the contents of the draft proposal
    const quoteCard = page.getByTestId('quote-draft-card').first();
    await expect(quoteCard).toContainText('Draft Quote');
    await expect(quoteCard).toContainText('Calculated Total:');
    await expect(quoteCard).toContainText('Scope of Work:');

    // Action buttons check
    const approveBtn = page.getByTestId('approve-quote-draft').first();
    await expect(approveBtn).toBeVisible();

    const rejectBtn = page.getByTestId('reject-proposal').first();
    await expect(rejectBtn).toBeVisible();

    // Click the "Approve & Send" button
    await approveBtn.click();

    // Wait for the card to disappear or success state
    await expect(approveBtn).not.toBeVisible({ timeout: 10000 });
  });
});
