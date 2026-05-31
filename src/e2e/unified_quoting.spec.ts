import { test, expect } from './fixtures';

test.describe('Unified Quoting & Dynamic Invoicing', () => {
  test('A non-technical owner can approve an AI-generated quote from the inbox', async ({ page }) => {
    // 1. Log in as owner (implicit via fixtures)
    // 2. Navigate to the Team / Inbox area
    await page.goto('/team');

    // Check that The Salesperson has pending items
    const salesCard = page.locator('button:has-text("The Salesperson")');
    await expect(salesCard).toBeVisible();
    await expect(salesCard).toContainText('awaiting approval');

    // 3. Open The Salesperson's inbox
    await salesCard.click();

    // Verify the inbox title
    await expect(page.getByRole('heading', { name: 'The Salesperson' }).first()).toBeVisible();

    // 4. Find the quote generation request
    // According to our seed data: "Drafted quote based on: "How much for custom cake for 20?""
    const quoteCard = page.locator('div.bg-white:has-text("How much for custom cake for 20?")');
    await expect(quoteCard).toBeVisible();

    // Verify UI components specific to the quote card
    await expect(quoteCard.getByText('Generated Quote')).toBeVisible();
    await expect(quoteCard.getByText('Total Amount:')).toBeVisible();
    await expect(quoteCard.getByText('$200')).toBeVisible();
    await expect(quoteCard.getByText('Required Deposit:')).toBeVisible();
    await expect(quoteCard.getByText('$100')).toBeVisible();

    // 5. Open the Edit Modal
    await quoteCard.getByRole('button', { name: 'Edit' }).click();

    // Verify modal is open
    const editModal = page.locator('.absolute.inset-0', { hasText: 'Edit Quote' });
    await expect(editModal).toBeVisible();
    await expect(editModal.getByPlaceholder('e.g., Make the deposit $100 instead')).toBeVisible();

    // Close modal
    await editModal.getByRole('button', { name: 'Cancel' }).click();

    // 6. Approve the quote
    await quoteCard.getByRole('button', { name: 'Send Quote & Payment Link' }).click();

    // Wait for the optimistic UI update
    await expect(quoteCard).not.toBeVisible();
  });
});
