import { randomUUID } from 'node:crypto';
import { test, expect, E2E_UNLIMITED_ADMIN_USER } from './fixtures';
import { createOwnerQuote } from './playwright/quote_fixture';
import { e2eDbQuery } from './db_utils';

test.describe('Persisted quote detail', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('owner reviews, edits and approves owned persisted terms without claiming customer delivery', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    const { quoteId } = await createOwnerQuote(page, adminUser.organizationId, { description: 'Owner-reviewed repair', priceCents: 15000 });
    await page.goto(`/quotes/${quoteId}`);
    await expect(page.getByRole('heading', { name: 'Review Estimate' })).toBeVisible();
    await expect(page.getByText('Owner-reviewed repair (x1)')).toBeVisible();
    await expect(page.getByText('$10.00', { exact: true })).toBeVisible();
    const observed = await page.request.get(`/api/v1/quotes/${quoteId}`);
    expect(observed.status()).toBe(200);
    const reviewedVersion = (await observed.json()).quote.updated_at;
    expect(reviewedVersion).toEqual(expect.any(String));
    await page.getByRole('button', { name: 'Edit quote' }).click();
    await page.getByLabel('Unit price for Owner-reviewed repair').fill('200');
    await page.getByLabel('Total amount').fill('200');
    // Changing a line price never invents a new deposit policy.
    await expect(page.getByLabel('Required deposit')).toHaveValue('10.00');
    const save = page.waitForResponse(response => new URL(response.url()).pathname === `/api/v1/quotes/${quoteId}` && response.request().method() === 'PUT');
    await page.getByRole('button', { name: 'Save Changes' }).click();
    const savedResponse = await save;
    expect(savedResponse.status()).toBe(200);
    expect(savedResponse.request().postDataJSON().expected_updated_at).toBe(reviewedVersion);
    const committedVersion = (await savedResponse.json()).updated_at;
    expect(committedVersion).toEqual(expect.any(String));
    expect(committedVersion).not.toBe(reviewedVersion);
    await expect(page.getByText('Quote changes saved.')).toBeVisible();
    const approval = page.waitForResponse(response => new URL(response.url()).pathname === `/api/v1/quotes/${quoteId}/approve` && response.request().method() === 'PATCH');
    await page.getByRole('button', { name: 'Approve quote', exact: true }).click();
    const approvedResponse = await approval;
    expect(approvedResponse.status()).toBe(200);
    expect(approvedResponse.request().postDataJSON()).toEqual({ expected_updated_at: committedVersion });
    await expect(page.getByText('SENT', { exact: true })).toBeVisible();
    await expect(page.getByText('Quote approval saved. Message delivery to the customer is not confirmed.')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Approve quote', exact: true })).toHaveCount(0);
    const read = await page.request.get(`/api/v1/quotes/${quoteId}`);
    expect(read.status()).toBe(200);
    expect(await read.json()).toMatchObject({ quote: { id: quoteId, status: 'SENT', total_amount_cents: 20000, required_deposit_cents: 1000, stripe_payment_link: null }, line_items: [expect.objectContaining({ description: 'Owner-reviewed repair', unit_price_cents: 20000 })], acceptance: null });
    const [persisted] = await e2eDbQuery('SELECT status,total_amount_cents,required_deposit_cents,stripe_payment_link FROM quotes WHERE id=$1 AND tenant_id=$2', [quoteId, adminUser.organizationId]);
    expect(persisted).toMatchObject({ status: 'SENT', stripe_payment_link: null });
    expect(BigInt(persisted.total_amount_cents)).toBe(20000n);
    expect(BigInt(persisted.required_deposit_cents)).toBe(1000n);
  });

  test('missing, foreign-tenant and legacy synthetic IDs never reveal or invent quote details', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, E2E_UNLIMITED_ADMIN_USER);
    const { quoteId } = await createOwnerQuote(page, E2E_UNLIMITED_ADMIN_USER.organizationId, { description: 'Foreign confidential quote', priceCents: 67890 });
    await loginAs(page, adminUser);
    for (const id of [randomUUID(), quoteId, 'e2e-id']) {
      await page.goto(`/quotes/${id}`);
      await expect(page.getByText('Quote not found', { exact: true })).toBeVisible();
      await expect(page.getByRole('button', { name: /Approve|Edit quote/i })).toHaveCount(0);
      await expect(page.getByText(/Foreign confidential quote|Sink Repair and Pipe Replacement|\$350.00/)).toHaveCount(0);
    }
    const foreign = await page.request.get(`/api/v1/quotes/${quoteId}`);
    expect(foreign.status()).toBe(404);
  });

  test('drafting remains pending until a refresh reads persisted ready terms', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    const { quoteId } = await createOwnerQuote(page, adminUser.organizationId, { description: 'Prepared quote terms', priceCents: 12500 });
    await e2eDbQuery("UPDATE quotes SET status='DRAFTING' WHERE id=$1 AND tenant_id=$2", [quoteId, adminUser.organizationId]);
    await page.goto(`/quotes/${quoteId}`);
    await expect(page.getByText(/Quote preparation is pending/)).toBeVisible();
    await expect(page.getByRole('button', { name: /Approve|Edit quote/i })).toHaveCount(0);
    await e2eDbQuery("UPDATE quotes SET status='DRAFT' WHERE id=$1 AND tenant_id=$2", [quoteId, adminUser.organizationId]);
    await page.getByRole('button', { name: 'Refresh quote' }).click();
    await expect(page.getByText('Prepared quote terms (x1)')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Approve quote', exact: true })).toBeEnabled();
  });

  test('stale reviewed terms cannot be approved after another persisted edit', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    const { quoteId } = await createOwnerQuote(page, adminUser.organizationId, { description: 'Initially reviewed terms', priceCents: 5000 });
    await page.goto(`/quotes/${quoteId}`);
    await expect(page.getByText('Initially reviewed terms (x1)')).toBeVisible();
    await e2eDbQuery('UPDATE quotes SET total_amount_cents=9000 WHERE id=$1 AND tenant_id=$2', [quoteId, adminUser.organizationId]);
    const approval = page.waitForResponse(response => new URL(response.url()).pathname === `/api/v1/quotes/${quoteId}/approve` && response.request().method() === 'PATCH');
    await page.getByRole('button', { name: 'Approve quote', exact: true }).click();
    const rejected = await approval;
    expect(rejected.status()).toBe(409);
    expect(await rejected.json()).toMatchObject({ success: false, reason: 'reviewed_quote_version_required' });
    await expect(page.getByText(/Approval could not be confirmed/)).toBeVisible();
    await expect(page.getByRole('button', { name: 'Approve quote', exact: true })).toBeDisabled();
    const [persisted] = await e2eDbQuery('SELECT status,total_amount_cents,stripe_payment_link FROM quotes WHERE id=$1 AND tenant_id=$2', [quoteId, adminUser.organizationId]);
    expect(persisted).toMatchObject({ status: 'DRAFT', stripe_payment_link: null });
    expect(BigInt(persisted.total_amount_cents)).toBe(9000n);
    await page.getByRole('button', { name: 'Refresh quote' }).click();
    await expect(page.getByText('$90.00', { exact: true })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Approve quote', exact: true })).toBeEnabled();
  });

  test('approval losing tenant authority stays unconfirmed and reconciles to missing', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    const { quoteId } = await createOwnerQuote(page, adminUser.organizationId, { description: 'Quote before ownership changed' });
    await page.goto(`/quotes/${quoteId}`);
    await expect(page.getByText('Quote before ownership changed (x1)')).toBeVisible();
    await e2eDbQuery('UPDATE quotes SET tenant_id=$1 WHERE id=$2 AND tenant_id=$3', [E2E_UNLIMITED_ADMIN_USER.organizationId, quoteId, adminUser.organizationId]);
    const approval = page.waitForResponse(response => new URL(response.url()).pathname === `/api/v1/quotes/${quoteId}/approve` && response.request().method() === 'PATCH');
    await page.getByRole('button', { name: 'Approve quote', exact: true }).click();
    expect((await approval).status()).toBe(404);
    await expect(page.getByText(/Approval could not be confirmed/)).toBeVisible();
    await expect(page.getByText('SENT', { exact: true })).toHaveCount(0);
    await expect(page.getByRole('button', { name: 'Approve quote', exact: true })).toBeDisabled();
    const [persisted] = await e2eDbQuery('SELECT status FROM quotes WHERE id=$1', [quoteId]);
    expect(persisted.status).toBe('DRAFT');
    await page.getByRole('button', { name: 'Refresh quote' }).click();
    await expect(page.getByText('Quote not found', { exact: true })).toBeVisible();
  });
});
