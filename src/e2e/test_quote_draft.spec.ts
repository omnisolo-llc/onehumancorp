import { test, expect } from '@playwright/test';

test.describe('Quote & Booking Agent Drafts', () => {
  test('User can review and approve an AI drafted quote in the team chat', async ({ page }) => {
    // Navigate to the Team Chat Page
    await page.goto('http://localhost:3000/team/chat');

    // Simulate finding a quote draft from the AI
    const quoteChip = await page.getByTestId('quote-draft-chip').first();
    await quoteChip.waitFor({ state: 'visible' });

    // Click on the chip to open QuoteReviewModal
    await quoteChip.click();

    // Verify QuoteReviewModal opens
    const modalContent = await page.getByText('Quote Review');
    await expect(modalContent).toBeVisible();

    // Verify that the "Approve" button exists inside the modal
    const approveBtn = await page.getByRole('button', { name: /Approve/i });
    await expect(approveBtn).toBeVisible();
  });
});

test.describe('Daily Work Triage Feed', () => {
  test('User can see and approve AI Summary actions', async ({ page }) => {
    // Navigate to Triage Feed Page
    await page.goto('http://localhost:3000/triage');

    // Assume there is an action card
    const cardHeader = await page.getByTestId(/triage-card-header-*/).first();
    await expect(cardHeader).toBeVisible();

    // Click to expand
    await cardHeader.click();

    // Look for AI Summary & Proposed Action text
    await expect(page.getByText(/AI Summary & Proposed Action:/i)).toBeVisible();

    // Look for Approve and Reject buttons
    const approveButton = await page.getByTestId(/triage-approve-*/).first();
    const rejectButton = await page.getByTestId(/triage-dismiss-*/).first();

    await expect(approveButton).toBeVisible();
    await expect(approveButton).toHaveText('Approve');

    await expect(rejectButton).toBeVisible();
    await expect(rejectButton).toHaveText('Reject');
  });
});
