import { test, expect } from '../fixtures';
import { e2eDbQuery } from '../db_utils';
import { createOwnerQuote } from './quote_fixture';

test.describe('Successful quote acceptance referral', () => {
  test('shows an attributed referral only after acceptance and an unpaid invoice persist', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    const { quoteId, customerId } = await createOwnerQuote(page, adminUser.organizationId, {
      description: 'Referral acceptance service',
    });
    await page.goto(`/ui/quote.html?id=${quoteId}`);
    await expect(page.locator('#line-items-container')).toContainText('Referral acceptance service');
    const card = page.getByTestId('referral-success-card');
    await expect(card).toBeHidden();
    const acceptedPromise = page.waitForResponse((response) =>
      new URL(response.url()).pathname === `/api/v1/quotes/${quoteId}/accept`
      && response.request().method() === 'POST');
    await page.getByRole('button', { name: 'Accept quote', exact: true }).click();
    const acceptedResponse = await acceptedPromise;
    expect(acceptedResponse.ok(), await acceptedResponse.text()).toBe(true);
    const accepted = await acceptedResponse.json();
    expect(accepted).toMatchObject({ success: true, status: 'accepted', quote_id: quoteId, invoice_status: 'Draft', payment_status: 'unverified', checkout_status: 'not_configured' });
    expect(acceptedResponse.request().postDataJSON().expected_updated_at).toEqual(expect.any(String));
    expect(accepted.invoice_id).toMatch(/^[0-9a-f-]{36}$/);
    // Native CI has no configured payment provider. Quote acceptance is durable,
    // but a draft invoice and empty payment link are never a collected deposit.
    expect(accepted.stripe_payment_link).toBe('');
    await expect(page.locator('#quote-acceptance')).toBeVisible();
    await expect(page.locator('#quote-status')).toHaveText('Accepted — payment pending');
    await expect(page.locator('#continue-payment')).toBeHidden();
    await expect(page.locator('body')).not.toContainText('Deposit Paid!');
    const [invoice] = await e2eDbQuery(
      'SELECT tenant_id, customer_id, quote_id, status, total_amount, total_amount_cents, stripe_invoice_id, stripe_payment_link, payment_status FROM invoices WHERE id = $1',
      [accepted.invoice_id],
    );
    expect(invoice).toMatchObject({
      tenant_id: adminUser.organizationId, customer_id: customerId,
      quote_id: quoteId, status: 'Draft', stripe_invoice_id: null, stripe_payment_link: null, payment_status: 'unverified', total_amount_cents: 5000,
    });
    expect(Number(invoice.total_amount)).toBe(50);
    const quote = await page.request.get(`/api/v1/quotes/${quoteId}`);
    expect(quote.ok()).toBe(true);
    expect(await quote.json()).toMatchObject({ quote: { id: quoteId, status: 'ACCEPTED' } });

    // Reload reads the accepted quote rather than accepting it a second time.
    await page.reload();
    await expect(page.locator('#quote-status')).toHaveText('Accepted — payment pending');
    await expect(card).toBeVisible();
    await expect(card).toContainText('Start Your AI Business');
    const link = card.getByRole('link', { name: 'Get OmniSolo Free' });
    await expect(link).toHaveAttribute('href', `/setup.html?ref=${encodeURIComponent(adminUser.organizationId)}&source=quote_accepted`);
    await link.click();
    await page.waitForURL((url) => url.pathname === '/setup.html'
      && url.searchParams.get('ref') === adminUser.organizationId
      && url.searchParams.get('source') === 'quote_accepted');
    await expect(page).toHaveTitle(/Setup/);
    expect(await e2eDbQuery('SELECT id FROM invoices WHERE quote_id = $1', [quoteId]))
      .toEqual([{ id: accepted.invoice_id }]);
  });
});
