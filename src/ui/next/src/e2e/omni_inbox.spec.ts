import { test, expect } from '@playwright/test';

test.describe('Omnichannel Inbox', () => {
  test('Maya can view and approve AI draft in inbox', async ({ page }) => {
    // Navigate to the Omni Inbox page
    await page.goto('/omni-inbox');

    // Wait for the inbox to load
    await expect(page.getByText('Inbox')).toBeVisible();
    await expect(page.getByText('1 Urgent')).toBeVisible();

    // Verify conversation list
    const convMaya = page.getByTestId('conv-1');
    await expect(convMaya).toBeVisible();
    await expect(convMaya.getByText('Maya Baker')).toBeVisible();
    await expect(convMaya.getByText('AI: Draft ready')).toBeVisible();

    // Select conversation
    await convMaya.click();

    // Verify conversation view opens
    await expect(page.getByRole('heading', { name: 'Maya Baker' })).toBeVisible();
    await expect(page.getByText('Do you do vegan cakes?')).toBeVisible();

    // Verify AI Draft is present
    await expect(page.getByText('AI Draft')).toBeVisible();
    await expect(page.getByText('Hi Maya! Yes, we absolutely do vegan cakes. Would you like a quote?')).toBeVisible();

    // Approve the draft
    const approveButton = page.getByTestId('approve-draft-m1');
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // Verify draft is removed and (in a real app) message is sent
    await expect(page.getByText('AI Draft')).not.toBeVisible();
  });
});
