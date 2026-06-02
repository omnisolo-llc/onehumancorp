import { test, expect } from '@playwright/test';

test.describe('Autonomous AI Omnichannel Invoicing Engine', () => {
  test('Carlos the Handyman generates an invoice from natural language', async ({ page }) => {
    // Navigate to the invoices page as a logged-in user
    await page.goto('/invoices');

    // Carlos types what he did into the natural language prompt
    const promptInput = page.locator('textarea#invoice-prompt');
    await expect(promptInput).toBeVisible();
    await promptInput.fill('Send an invoice for $150 to Carlos for fixing the sink');

    // Carlos clicks the generate button
    const generateBtn = page.locator('[data-testid="generate-invoice-btn"]');
    await expect(generateBtn).toBeEnabled();
    await generateBtn.click();

    // The AI takes over, parses the request, creates a stripe link, and shows the invoice preview
    const paymentLinkBtn = page.locator('[data-testid="payment-link-btn"]');
    await expect(paymentLinkBtn).toBeVisible({ timeout: 10000 });

    // Verify the invoice preview contains the correct data parsed from the prompt
    await expect(page.locator('text=$150')).toBeVisible();

    // The payment link should link to a Stripe checkout session
    const href = await paymentLinkBtn.getAttribute('href');
    expect(href).toContain('stripe.com');
  });
});
