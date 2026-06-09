import { test, expect } from './fixtures';

test.describe('Sales Assistant Quoting Journey', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('full journey: inquiry -> draft -> approval -> customer accept', async ({ adminUser, loginAs, page }) => {
    // 1. Login as Owner
    await loginAs(page, adminUser);
    await page.goto('/dashboard');

    // 2. Owner sees the draft quote card in the agent feed (seeded data)
    const draftCard = page.locator('[data-testid="draft-quote-card"]');
    await expect(draftCard).toBeVisible();
    await expect(draftCard).toContainText('Draft Quote Ready');

    // 3. Owner clicks "Edit Draft" to review
    await page.locator('[data-testid="edit-quote-draft"]').click();
    await expect(page).toHaveURL(/\/quoting\?id=.*/);
    await expect(page.getByText('Review Draft Quote')).toBeVisible();

    // 4. Owner reviews line items and adds a custom item
    await expect(page.getByText('Fix leaking sink')).toBeVisible();

    // Simulate adding a custom item (via prompt in our mock-ish frontend implementation)
    // Note: Playwright handles dialogs
    page.on('dialog', async dialog => {
      if (dialog.message().includes('description')) {
        await dialog.accept('Parts & Materials');
      } else {
        await dialog.accept('25');
      }
    });

    await page.getByRole('button', { name: 'Add custom item' }).click();
    await expect(page.getByText('Parts & Materials')).toBeVisible();

    // 5. Owner approves the quote
    await page.getByRole('button', { name: 'Approve & Send' }).click();
    await expect(page.getByText('Sent to Customer')).toBeVisible();

    // 6. Get the quote ID from the URL and visit the public proposal page
    const url = new URL(page.url());
    const quoteId = url.searchParams.get('id');

    // Switch to "Customer" perspective
    await page.goto(`/proposal/${quoteId}`);

    // 7. Customer views the beautiful proposal
    await expect(page.getByText('Service Proposal')).toBeVisible();
    await expect(page.getByText('Fix leaking sink')).toBeVisible();
    await expect(page.getByText('Parts & Materials')).toBeVisible();

    // 8. Customer accepts the proposal
    await page.getByRole('button', { name: 'Accept & Pay Deposit' }).click();

    // 9. Verify success state
    await expect(page.getByText('Proposal Accepted!')).toBeVisible();
  });
});
