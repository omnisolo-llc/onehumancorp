import { startConfiguredCheckoutFixture as startOwnedRuntime } from '../../../scripts/checkout-browser-fixture.mjs';
import { e2eDbQuery } from '../db_utils';

export type CheckoutRegistration = { tenantId: string; productId: string; title: string; amountCents: number };

/** Session creation + stock exclusion only. No hosted payment or paid event. */
export async function startConfiguredCheckoutFixture() {
  const runtime = await startOwnedRuntime();
  return {
    ...runtime,
    async register(product: CheckoutRegistration) {
      const rows = await e2eDbQuery(`SELECT p.id, p.tenant_id, p.title, p.price_cents::text AS price_cents
        FROM products p JOIN tenants t ON t.id=p.tenant_id WHERE p.id=$1 AND p.tenant_id=$2`,
      [product.productId, product.tenantId]);
      if (rows.length !== 1 || rows[0].id !== product.productId || rows[0].tenant_id !== product.tenantId
          || rows[0].title !== product.title || rows[0].price_cents !== String(product.amountCents)) {
        throw new Error('The registered checkout terms must match the runner-owned tenant/product in PostgreSQL');
      }
      runtime.register(product);
    },
  };
}
