import { test, expect } from '@playwright/test';
import { currentAppSmoke } from './fixtures';

currentAppSmoke('omnichannel_invoicing_engine');

test.describe('Autonomous AI Omnichannel Invoicing Engine', () => {
  test('creates an invoice from natural language and generates payment link', async ({ page }) => {
    await page.goto('/');

    await page.getByRole('button', { name: 'Invoice', exact: true }).first().click();

    await expect(page.getByRole('heading', { name: 'Create Invoice' })).toBeVisible();

    const invoicePrompt = "Send an invoice for $150 to John for ceiling fan installation";
    await page.getByPlaceholder('e.g. Send an invoice for $50').fill(invoicePrompt);

    await page.getByRole('button', { name: '✨ Generate Quote & Payment Link' }).click();

    const previewCard = page.locator('#invoice-preview');
    await expect(previewCard).toBeVisible();

    await expect(previewCard.locator('#preview-customer')).toContainText('John');
    await expect(previewCard.locator('#preview-amount')).toContainText('150');
    await expect(previewCard.locator('#preview-items')).toContainText(invoicePrompt);

    const paymentLink = previewCard.locator('#preview-link');
    await expect(paymentLink).toHaveAttribute('href', /https:\/\/buy\.stripe\.com\/test_.*/);
    await expect(paymentLink).toContainText('buy.stripe.com/test');
  });
});
