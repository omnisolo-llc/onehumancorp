import { test, expect } from './fixtures';
import { startConfiguredCheckoutFixture } from './support/configured_checkout_fixture';
import {
  CASH_COMMIT_PATH, CHECKOUT_SESSION_PATH, createOwnedCheckoutActors, prepareCashCart, prepareOnlineCart,
  waitForCheckoutPost, assertCashReceipt, assertCashRejection, assertPersistedStockOutcome,
} from './support/owned_checkout_stock';

// These cases exercise real cash completion and provider-session stock holds.
// Hosted Stripe payment and paid-webhook completion remain a separate boundary.
test.describe('Owned cash and online checkout session stock exclusion', () => {
  test.describe.configure({ mode: 'default' });
  let fixture: Awaited<ReturnType<typeof startConfiguredCheckoutFixture>>;

  test.beforeAll(async () => { fixture = await startConfiguredCheckoutFixture(); });
  test.afterAll(async () => { await fixture?.close(); });
  test.afterEach(async ({ browserName }, testInfo) => {
    if (fixture) await testInfo.attach(`configured-checkout-provider-evidence-${browserName}`, {
      body: JSON.stringify(fixture.evidence(), null, 2), contentType: 'application/json',
    });
  });

  test('two cash terminals complete exactly one persisted sale of the owned last unit', async ({ browser, contextOptions }) => {
    const actors = await createOwnedCheckoutActors(browser, fixture.origin, { ...contextOptions, proxy: fixture.proxy });
    try {
      await prepareCashCart(actors.first, actors.stock, actors.firstUserId);
      await prepareCashCart(actors.second, actors.stock, actors.secondUserId);
      const firstResult = waitForCheckoutPost(actors.first, CASH_COMMIT_PATH);
      const secondResult = waitForCheckoutPost(actors.second, CASH_COMMIT_PATH);
      await Promise.all([actors.first.locator('#cash-btn-offline').click(), actors.second.locator('#cash-btn-offline').click()]);
      const responses = await Promise.all([firstResult, secondResult]);
      expect(responses.map(response => response.status()).sort()).toEqual([200, 409]);
      const winner = responses[0].status() === 200 ? 0 : 1;
      const pages = [actors.first, actors.second];
      const receipt = await assertCashReceipt(responses[winner], pages[winner], actors.stock);
      await assertCashRejection(responses[1 - winner], pages[1 - winner]);
      await assertPersistedStockOutcome(actors.stock, receipt.order_id);
      expect(fixture.evidence().requests.filter(request => request.form?.client_reference_id === actors.stock.tenantId)).toEqual([]);
    } finally { await actors.close(); }
  });

  for (const ordering of ['online-first', 'concurrent'] as const) {
    test(`${ordering} cash and online session attempts preserve exactly one last-unit allocation`, async ({ browser, contextOptions }) => {
      const actors = await createOwnedCheckoutActors(browser, fixture.origin, { ...contextOptions, proxy: fixture.proxy });
      const redirects: string[] = [];
      const previousCheckoutConnects = fixture.evidence().connects.filter(request => request.authority === 'checkout.stripe.com:443').length;
      actors.second.on('request', request => {
        if (new URL(request.url()).hostname === 'checkout.stripe.com') redirects.push(request.url());
      });
      try {
        await fixture.register(actors.stock);
        await prepareCashCart(actors.first, actors.stock, actors.firstUserId);
        await prepareOnlineCart(actors.second, actors.stock);
        const cashResult = waitForCheckoutPost(actors.first, CASH_COMMIT_PATH);
        const onlineResult = waitForCheckoutPost(actors.second, CHECKOUT_SESSION_PATH);
        if (ordering === 'online-first') {
          await actors.second.getByRole('button', { name: 'Pay', exact: true }).click();
          const held = await onlineResult;
          expect(held.status()).toBe(200);
          await held.body();
          await assertPersistedStockOutcome(actors.stock, null);
          await actors.first.locator('#cash-btn-offline').click();
        } else {
          await Promise.all([
            actors.first.locator('#cash-btn-offline').click(),
            actors.second.getByRole('button', { name: 'Pay', exact: true }).click(),
          ]);
        }
        const [cash, online] = await Promise.all([cashResult, onlineResult]);
        expect([cash.status(), online.status()].sort()).toEqual([200, 409]);
        const requestBody = online.request().postDataJSON();
        expect(requestBody).toEqual({ is_subscription: false, product_id: actors.stock.productId, quantity: 1 });
        const providerRequests = fixture.evidence().requests.filter(request => request.form?.client_reference_id === actors.stock.tenantId);
        if (cash.status() === 200) {
          const receipt = await assertCashReceipt(cash, actors.first, actors.stock);
          expect(online.status()).toBe(409);
          await online.body();
          await expect(actors.second.getByText('The selected product just sold out.', { exact: true })).toBeVisible();
          expect(providerRequests).toEqual([]);
          expect(redirects).toEqual([]);
          await assertPersistedStockOutcome(actors.stock, receipt.order_id);
        } else {
          await assertCashRejection(cash, actors.first);
          expect(providerRequests).toHaveLength(1);
          const issued = providerRequests[0];
          expect(issued).toMatchObject({ method: 'POST', path: '/v1/checkout/sessions', status: 200,
            form: { client_reference_id: actors.stock.tenantId, 'metadata[product_id]': actors.stock.productId,
              'line_items[0][price_data][unit_amount]': String(actors.stock.amountCents), 'line_items[0][quantity]': '1' },
            receipt: { payment_status: 'unpaid', amount_total: actors.stock.amountCents, currency: 'usd' } });
          expect(issued.receipt.id).toMatch(/^cs_test_[a-f0-9_]+$/);
          const checkoutUrl = `https://checkout.stripe.com/c/pay/${issued.receipt.id}`;
          expect(await online.json()).toEqual({ checkout_url: checkoutUrl });
          await expect.poll(() => redirects).toContain(checkoutUrl);
          await expect.poll(() => fixture.evidence().connects.filter(request => request.authority === 'checkout.stripe.com:443').length).toBeGreaterThan(previousCheckoutConnects);
          const appAuthority = new URL(fixture.origin).host;
          expect(fixture.evidence().connects.some(request => request.authority === appAuthority && request.status === 200)).toBe(true);
          expect(fixture.evidence().connects.every(request => request.status === (request.authority === appAuthority ? 200 : 403))).toBe(true);
          await assertPersistedStockOutcome(actors.stock, null);
        }
      } finally { await actors.close(); }
    });
  }
});

test.describe('Inventory and POS entry screens', () => {
  test('verify online checkout page UI renders gracefully', async ({ page }) => {
    await page.goto('/checkout?product_id=prod_test');
    await expect(page.getByText('Secure Checkout')).toBeVisible();
  });

  test('verify POS terminal offline UI renders', async ({ page }) => {
    await page.goto('/pos/terminal');
    await expect(page.locator('#pos-keypad')).toBeVisible();
  });

  test('verify KDS terminal renders', async ({ page }) => {
    await page.goto('/pos/kds');
    await expect(page.locator('body')).toBeVisible();
  });

  test('verify inventory page renders', async ({ page }) => {
    await page.goto('/inventory');
    await expect(page.locator('body')).toBeVisible();
  });
});
