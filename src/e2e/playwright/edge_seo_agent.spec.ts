import { randomUUID } from 'node:crypto';
import { test, expect } from '../fixtures';
import { e2eDbQuery } from '../db_utils';
import { publishOwnedStorefront } from '../published_storefront_fixture';
import type { SiteSnapshot } from '../../ui/next/src/app/builder/publicationContracts';

function productSchema(html: string): Record<string, unknown> {
  const script = html.match(/<script type="application\/ld\+json">\s*([\s\S]*?)\s*<\/script>/);
  expect(script, 'The public storefront must contain parseable JSON-LD').not.toBeNull();
  return JSON.parse(script![1]) as Record<string, unknown>;
}

test.describe('Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering', () => {
  test('Marketing Agent refreshes private product SEO while reviewed public snapshots stay immutable', async ({ page, request, memberPage, anonymousPage, baseURL }) => {
    if (!process.env.E2E_POSTGRES_CONTAINER?.startsWith('ohc-e2e-pg-')) {
      throw new Error('Storefront fixtures require the native isolated E2E PostgreSQL container.');
    }
    const apiOrigin = process.env.API_BASE_URL;
    if (!apiOrigin) throw new Error('The native backend API_BASE_URL is required for public cache checks.');
    const tenantId = randomUUID();
    const userId = randomUUID();
    const email = `edge-${userId}@example.test`;
    const productName = `SEO Edge Cake ${randomUUID()}`;
    const description = 'A beautifully edge-cached vegan cake.';
    const storeName = `Edge bakery ${tenantId}`;

    // The native runner drops this isolated database after the suite. Keep
    // UUID-scoped fixtures until then so asynchronous jobs retain their rows.
    // Only prerequisites are fixtures. Product create/edit, publication and SEO
    // below must go through the real mounted application and agent handlers.
    await e2eDbQuery(
      "INSERT INTO tenants (id, name, industry, tier, plan_tier) VALUES ($1, $2, 'Food and beverage', 'Pro', 'Pro')",
      [tenantId, storeName],
    );
    const users = await e2eDbQuery(
      `WITH fixture_user AS (
         INSERT INTO users (id, username, email, password_hash, roles, active, tenant_id)
         SELECT $1, $2, $2, password_hash, ARRAY['ADMIN'], true, $3
         FROM users WHERE id = 'e2e-admin-user' RETURNING id, tenant_id
       )
       INSERT INTO identity_user_roles (user_id, role_name, tenant_id, position)
       SELECT id, 'ADMIN', tenant_id, 0 FROM fixture_user RETURNING user_id`,
      [userId, email, tenantId],
    );
    expect(users).toHaveLength(1);
    await page.context().clearCookies();
    await page.goto('/login');
    await page.getByLabel('Email or username').fill(email);
    await page.getByLabel('Password', { exact: true }).fill('password123');
    await page.getByLabel('Organization', { exact: true }).fill(tenantId);
    await page.getByRole('button', { name: 'Sign in', exact: true }).click();
    await expect(page).toHaveURL(/\/dashboard(?:[/?]|$)/);

    const mutationHeaders = { origin: new URL(page.url()).origin, 'sec-fetch-site': 'same-origin' };
    const privateDraft = await page.request.post('/api/v1/builder/publish_draft', {
      headers: mutationHeaders,
      data: {
        domain: null,
        draft: {
          domain: null,
          pages: [{
            path: '/', title: storeName,
            seo_metadata: { '@context': 'https://schema.org', '@type': 'Store', name: storeName },
            blocks: [{ block_type: 'HeroBlock', sort_order: 0, content: { headline: storeName, subtitle: 'Owner-published bakery' } }],
          }],
        },
      },
    });
    expect(privateDraft.status()).toBe(200);
    const site = await privateDraft.json() as { id: string };
    expect(site.id).toMatch(/^[a-f0-9-]{36}$/);
    await expect.poll(async () => {
      const rows = await e2eDbQuery('SELECT published_at IS NOT NULL AS published FROM builder_sites WHERE id = $1 AND tenant_id = $2', [site.id, tenantId]);
      return rows[0]?.published;
    }, { message: 'The legacy private-draft job must finish before previewing it', timeout: 15000 }).toBe(true);
    const siteResponse = await page.request.get(`/api/v1/builder/edge/${tenantId}/${site.id}`);
    expect(siteResponse.status()).toBe(200);
    expect(await siteResponse.text()).toContain(storeName);

    await page.goto('/products');
    await page.getByRole('button', { name: 'New Product', exact: true }).click();
    await page.getByLabel('Product Name').fill(productName);
    await page.getByLabel('Description', { exact: true }).fill(description);
    await page.getByLabel('Price', { exact: true }).fill('50.00');
    const createdResponse = page.waitForResponse((response) => response.url().endsWith('/api/v1/catalog/product') && response.request().method() === 'POST');
    await page.getByRole('button', { name: 'Save', exact: true }).click();
    const created = await createdResponse;
    expect(created.status()).toBe(200);
    const product = await created.json() as { success: boolean; product_id: string };
    expect(product.success).toBe(true);
    expect(product.product_id).toMatch(/^[a-f0-9-]{36}$/);
    await expect(page.getByRole('status').filter({ hasText: 'Product created' })).toBeVisible();
    await expect(page.getByRole('button', { name: `Edit ${productName}`, exact: true })).toBeVisible();

    const persistedProduct = async () => (await e2eDbQuery(
      'SELECT title, price_cents, seo_title, seo_schema_json FROM products WHERE id = $1 AND tenant_id = $2',
      [product.product_id, tenantId],
    ))[0];
    await expect.poll(async () => {
      const row = await persistedProduct();
      return { title: row?.title, price: Number(row?.price_cents), seoName: row?.seo_schema_json?.name, seoPrice: Number(row?.seo_schema_json?.offers?.price) };
    }, { message: 'The marketing agent must persist product-specific SEO from the actual creation event', timeout: 15000 }).toEqual({ title: productName, price: 5000, seoName: productName, seoPrice: 50 });

    const storefrontPath = `/api/v1/storefront/${tenantId}/${product.product_id}`;
    expect((await anonymousPage.request.get(new URL(storefrontPath, apiOrigin).href)).status()).toBe(401);
    expect((await request.get(storefrontPath)).status()).toBe(404);
    const first = await page.request.get(storefrontPath);
    expect(first.status()).toBe(200);
    expect(first.headers()['cache-control']).toContain('public, s-maxage=60, stale-while-revalidate=86400');
    expect(first.headers()['x-cache']).toBeUndefined();
    expect(productSchema(await first.text())).toMatchObject({ '@type': 'Product', name: productName, offers: { price: 50, priceCurrency: 'USD' } });
    const initialSeo = (await persistedProduct()).seo_schema_json as SiteSnapshot['pages'][number]['seo_metadata'];
    const reviewedSnapshot = (seo: SiteSnapshot['pages'][number]['seo_metadata']): SiteSnapshot => ({
      domain: null, pages: [{ path: '/', title: productName, seo_metadata: seo,
        blocks: [{ block_type: 'HeroBlock', sort_order: 0, content: { headline: productName, subtitle: description } }],
      }],
    });
    const published = await publishOwnedStorefront(page, baseURL, { userId, tenantId }, reviewedSnapshot(initialSeo));
    const publicPath = published.receipt.public_path as string;
    const publicResponse = await anonymousPage.request.get(publicPath);
    expect(publicResponse.status()).toBe(200);
    expect(publicResponse.headers()['cache-control']).toContain('public, s-maxage=60, stale-while-revalidate=86400');
    expect(publicResponse.headers()['etag']).toBeTruthy();
    const publicHtml = await publicResponse.text();
    expect(productSchema(publicHtml)).toMatchObject({ '@type': 'Product', name: productName, offers: { price: 50, priceCurrency: 'USD' } });
    await anonymousPage.goto(publicPath);
    await expect(anonymousPage).toHaveTitle(productName);
    await expect(anonymousPage.locator('meta[name="description"]')).toHaveAttribute('content', new RegExp(description.replace('.', '\\.')));

    // Authorization, tenant isolation and invalid input must leave the real
    // saved row untouched.
    const edit = { name: productName, description, price: '45.00' };
    const endpoint = `/api/v1/catalog/product/${product.product_id}`;
    expect((await request.put(endpoint, { data: edit, headers: mutationHeaders })).status()).toBe(404);
    expect((await memberPage.request.put(endpoint, { data: edit, headers: mutationHeaders })).status()).toBe(403);
    expect((await anonymousPage.request.put(endpoint, { data: edit, headers: mutationHeaders })).status()).toBe(401);
    for (const price of ['-0.01', 'NaN', '1e100', '10000000.01', '45.001']) {
      expect((await page.request.put(endpoint, { data: { ...edit, price }, headers: mutationHeaders })).status()).toBe(400);
    }
    expect(Number((await persistedProduct()).price_cents)).toBe(5000);

    await page.goto('/products');
    await page.getByRole('button', { name: `Edit ${productName}`, exact: true }).click();
    await expect(page.getByLabel('Price', { exact: true })).toHaveValue('50.00');
    await page.getByLabel('Price', { exact: true }).fill('45.00');
    const updatedResponse = page.waitForResponse((response) => response.url().endsWith(endpoint) && response.request().method() === 'PUT');
    await page.getByRole('button', { name: 'Save', exact: true }).click();
    expect((await updatedResponse).status()).toBe(200);
    await expect(page.getByRole('status').filter({ hasText: 'Product updated' })).toBeVisible();
    await expect(page.getByText('$45.00', { exact: true })).toBeVisible();
    const invalidated = await page.request.get(storefrontPath);
    expect(invalidated.status()).toBe(200);
    expect(invalidated.headers()['cache-control']).toContain('public, s-maxage=60, stale-while-revalidate=86400');
    expect(invalidated.headers()['x-cache']).toBeUndefined();

    await expect.poll(async () => {
      const row = await persistedProduct();
      return { price: Number(row?.price_cents), seoPrice: Number(row?.seo_schema_json?.offers?.price) };
    }, { message: 'ProductUpdated must refresh persisted pricing and generated SEO', timeout: 15000 }).toEqual({ price: 4500, seoPrice: 45 });
    await expect.poll(async () => {
      const response = await page.request.get(storefrontPath);
      expect(response.status()).toBe(200);
      return productSchema(await response.text());
    }, { message: 'The owner-only product preview must use refreshed SEO', timeout: 15000 }).toMatchObject({ name: productName, offers: { price: 45 } });
    // Updating a private catalog is never fresh publication authority.
    const unchanged = await anonymousPage.request.get(publicPath);
    expect(unchanged.status()).toBe(200);
    expect(await unchanged.text()).toBe(publicHtml);
    expect(unchanged.headers()['etag']).toBe(publicResponse.headers()['etag']);
    const updatedSeo = (await persistedProduct()).seo_schema_json as SiteSnapshot['pages'][number]['seo_metadata'];
    const republished = await publishOwnedStorefront(page, baseURL, { userId, tenantId }, reviewedSnapshot(updatedSeo));
    const updatedPublic = await anonymousPage.request.get(republished.receipt.public_path);
    expect(updatedPublic.status()).toBe(200);
    expect(productSchema(await updatedPublic.text())).toMatchObject({ name: productName, offers: { price: 45 } });
    expect(republished.receipt.public_path).not.toBe(publicPath);
    await page.reload();
    await expect(page.getByText('$45.00', { exact: true })).toBeVisible();

  });
});
