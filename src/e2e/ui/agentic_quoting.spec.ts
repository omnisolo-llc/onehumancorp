import { test, expect } from '../fixtures';
import { createOwnerQuote } from '../playwright/quote_fixture';
import { e2eDbQuery } from '../db_utils';

test.describe('Quote review on mobile', () => {
  test('owner saves and approves reviewed terms, then customer acceptance records an unpaid invoice', async ({ page, adminUser, loginAs }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await loginAs(page, adminUser);
    const { quoteId } = await createOwnerQuote(page, adminUser.organizationId, {
      description: 'Fix leaking sink including labor and standard materials', priceCents: 15000,
    });
    const initiallyLoaded = page.waitForResponse(response => new URL(response.url()).pathname === `/api/v1/quotes/${quoteId}` && response.request().method() === 'GET');
    await page.goto(`/ui/quote.html?mode=owner&id=${quoteId}`);
    const initialVersion = (await (await initiallyLoaded).json()).quote.updated_at;
    expect(initialVersion).toEqual(expect.any(String));
    await expect(page.locator('#quote-total')).toHaveText('$150.00');
    await expect(page.locator('#deposit-amount')).toHaveText('$10.00');
    await expect(page.locator('.line-item-desc').first()).toHaveText('Fix leaking sink including labor and standard materials');
    await page.getByRole('button', { name: 'Edit Quote', exact: true }).click();
    await expect(page.getByRole('dialog')).toBeVisible();
    await page.locator('.edit-price').fill('200');
    await page.locator('#edit-total-amount').fill('200');
    await page.locator('#edit-required-deposit').fill('50');
    const saved = page.waitForResponse(response => new URL(response.url()).pathname === `/api/v1/quotes/${quoteId}` && response.request().method() === 'PUT');
    const savedReload = page.waitForResponse(response => new URL(response.url()).pathname === `/api/v1/quotes/${quoteId}` && response.request().method() === 'GET');
    await page.getByRole('button', { name: 'Save Changes', exact: true }).click();
    const saveResponse = await saved;
    expect(saveResponse.status()).toBe(200);
    expect(saveResponse.request().postDataJSON().expected_updated_at).toBe(initialVersion);
    const saveReceipt = await saveResponse.json();
    expect(saveReceipt).toMatchObject({ success: true, updated_at: expect.any(String) });
    const savedVersion = (await (await savedReload).json()).quote.updated_at;
    expect(savedVersion).toBe(saveReceipt.updated_at);
    expect(savedVersion).not.toBe(initialVersion);
    const [savedTerms] = await e2eDbQuery('SELECT updated_at=$1::timestamptz AS version_matches FROM quotes WHERE id=$2 AND tenant_id=$3', [savedVersion, quoteId, adminUser.organizationId]);
    expect(savedTerms.version_matches).toBe(true);
    await expect(page.getByRole('dialog')).toBeHidden();
    await expect(page.locator('#quote-total')).toHaveText('$200.00');
    const approved = page.waitForResponse(response => new URL(response.url()).pathname === `/api/v1/quotes/${quoteId}` && response.request().method() === 'PUT');
    await page.getByRole('button', { name: 'Approve quote', exact: true }).click();
    const approval = await approved;
    expect(approval.status()).toBe(200);
    expect(approval.request().postDataJSON()).toMatchObject({ expected_updated_at: savedVersion, status: 'SENT' });
    const approvalReceipt = await approval.json();
    expect(approvalReceipt).toMatchObject({ success: true, updated_at: expect.any(String) });
    expect(approvalReceipt.updated_at).not.toBe(savedVersion);
    await expect(page.getByText('Quote approval saved. Message delivery to the customer is not confirmed.')).toBeVisible();
    const [terms] = await e2eDbQuery('SELECT status,total_amount_cents,required_deposit_cents FROM quotes WHERE id=$1 AND tenant_id=$2', [quoteId, adminUser.organizationId]);
    expect(terms).toMatchObject({ status: 'SENT' });
    // PostgreSQL BIGINT values retain exact decimal strings in the fixture API.
    expect(BigInt(terms.total_amount_cents)).toBe(BigInt(20000));
    expect(BigInt(terms.required_deposit_cents)).toBe(BigInt(5000));

    const customerLoaded = page.waitForResponse(response => new URL(response.url()).pathname === `/api/v1/quotes/${quoteId}` && response.request().method() === 'GET');
    await page.goto(`/ui/quote.html?id=${quoteId}&mode=customer`);
    const customerVersion = (await (await customerLoaded).json()).quote.updated_at;
    expect(customerVersion).toBe(approvalReceipt.updated_at);
    await expect(page.locator('#quote-status')).toHaveText('Quote — review before accepting');
    const accepted = page.waitForResponse(response => new URL(response.url()).pathname === `/api/v1/quotes/${quoteId}/accept` && response.request().method() === 'POST');
    await page.getByRole('button', { name: 'Accept quote', exact: true }).click();
    const acceptance = await accepted;
    expect(acceptance.status()).toBe(200);
    expect(acceptance.request().postDataJSON().expected_updated_at).toBe(customerVersion);
    const receipt = await acceptance.json();
    expect(receipt).toMatchObject({ success: true, status: 'accepted', quote_id: quoteId, invoice_status: 'Draft', payment_status: 'unverified', checkout_status: 'not_configured', stripe_payment_link: '' });
    expect(receipt.invoice_id).toMatch(/^[0-9a-f-]{36}$/);
    await expect(page.locator('#quote-status')).toHaveText('Accepted — payment pending');
    await expect(page.locator('#continue-payment')).toBeHidden();
    await expect(page).toHaveURL(new RegExp(`/ui/quote\\.html\\?id=${quoteId}`));
    await expect(page.locator('body')).not.toContainText('Deposit Paid!');
    const [invoice] = await e2eDbQuery('SELECT status,payment_status,total_amount_cents,stripe_invoice_id,stripe_payment_link FROM invoices WHERE id=$1 AND quote_id=$2 AND tenant_id=$3', [receipt.invoice_id, quoteId, adminUser.organizationId]);
    expect(invoice).toMatchObject({ status: 'Draft', payment_status: 'unverified', stripe_invoice_id: null, stripe_payment_link: null });
    expect(BigInt(invoice.total_amount_cents)).toBe(BigInt(20000));
  });
});
