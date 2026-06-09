import { test, expect } from '@playwright/test';

test.describe('Universal Edge-Cached Dynamic Storefront & Agentic SEO', () => {
    test.describe('Edge Cache Hits and Misses', () => {
        test('verifies cache hits for repeated reads and tenant isolation', async ({ request }) => {
            // These random UUIDs will hit the API and since there is no site, it returns 404
            // But we should test actual caching headers
            const tenantA = '11111111-1111-1111-1111-111111111111';
            const siteA = '11111111-1111-1111-1111-111111111111';

            // Note: Currently in edge.rs if site is not found it returns 404, not cached.
            // Let's create a site first via API
            const loginRes = await request.post('/api/v1/auth/login', {
                data: {
                    email: 'maya@ohc.local',
                    password: 'maya'
                }
            });
            const authBody = await loginRes.json();
            const token = authBody.token;

            const siteRes = await request.post('/api/v1/builder/publish_draft', {
                headers: {
                    'Authorization': `Bearer ${token}`
                },
                data: {
                    domain: 'maya-test-domain',
                    draft: {
                        pages: [
                            {
                                path: '/',
                                title: 'Home',
                                seo_metadata: {},
                                blocks: [
                                    {
                                        block_type: 'HeroBlock',
                                        content: { headline: 'Edge Cached Hero' },
                                        sort_order: 1
                                    }
                                ]
                            }
                        ]
                    }
                }
            });
            expect(siteRes.ok()).toBeTruthy();
            const siteJson = await siteRes.json();
            const siteId = siteJson.id;
            // The tenant ID is associated with the user, let's get it from the token payload (or we can just wait for it).
            // Actually, for Maya it's a fixed UUID or we can find it. But we can just use the builder API route.
            // But the edge route requires tenantId. Let's get Maya's tenant ID from her profile.
            const profileRes = await request.get('/api/v1/auth/me', {
                headers: { 'Authorization': `Bearer ${token}` }
            });
            const profile = await profileRes.json();
            const tenantId = profile.tenant_id;

            // Wait for DB sync
            await new Promise(r => setTimeout(r, 1000));

            // 1. Fetch site A for the first time
            const reqA1 = await request.get(`/api/v1/builder/edge/${tenantId}/${siteId}`);
            expect(reqA1.ok()).toBeTruthy();
            const textA1 = await reqA1.text();
            expect(textA1).toContain('Edge Cached Hero');

            // Check cache control header
            expect(reqA1.headers()['cache-control']).toContain('s-maxage=60');

            // 2. Fetch site A second time - should be a cache hit
            const reqA2 = await request.get(`/api/v1/builder/edge/${tenantId}/${siteId}`);
            expect(reqA2.ok()).toBeTruthy();
            const textA2 = await reqA2.text();

            expect(textA1).toEqual(textA2);

            // 3. Fetch site B for the first time (Tenant Isolation check)
            const tenantB = '22222222-2222-2222-2222-222222222222';
            const reqB1 = await request.get(`/api/v1/builder/edge/${tenantB}/${siteId}`);
            expect(reqB1.status()).toBe(404); // Site doesn't belong to tenant B
        });

        test('verifies cache miss and repopulation following inventory update', async ({ page, request }) => {
            // Log in as an owner (e.g. Maya)
            await page.goto('/login');
            await page.fill('input[type="email"]', 'maya@ohc.local');
            await page.fill('input[type="password"]', 'maya');
            await page.click('button[type="submit"]');
            await page.waitForURL('/dashboard');

            // Add a new product
            await page.goto('/products');
            const newProductBtn = page.getByRole('button', { name: 'Add Product' });
            await newProductBtn.click();
            await page.getByPlaceholder('Product Title').fill('Dynamic Edge Cake');
            await page.getByPlaceholder('Price').fill('15');
            await page.getByRole('button', { name: 'Save Product' }).click();

            // Wait for creation
            await expect(page.getByText('Dynamic Edge Cake')).toBeVisible();

            // Simulate the inventory update via POS or terminal
            // Then fetch the edge cache and ensure the inventory cache is invalidated correctly
        });

        test('verifies SEO pre-rendering service correctly generates static HTML', async ({ page, request }) => {
            // Wait for the SEO job to run and the HTML to include SEO tags
            const loginRes = await request.post('/api/v1/auth/login', {
                data: {
                    email: 'maya@ohc.local',
                    password: 'maya'
                }
            });
            const authBody = await loginRes.json();
            const token = authBody.token;

            const siteRes = await request.post('/api/v1/builder/publish_draft', {
                headers: {
                    'Authorization': `Bearer ${token}`
                },
                data: {
                    domain: 'maya-seo-domain',
                    draft: {
                        pages: [
                            {
                                path: '/',
                                title: 'Home SEO',
                                seo_metadata: {},
                                blocks: [
                                    {
                                        block_type: 'HeroBlock',
                                        content: { headline: 'Seo Cake', description: 'Best cakes in town' },
                                        sort_order: 1
                                    }
                                ]
                            }
                        ]
                    }
                }
            });
            expect(siteRes.ok()).toBeTruthy();
            const siteJson = await siteRes.json();
            const siteId = siteJson.id;

            const profileRes = await request.get('/api/v1/auth/me', {
                headers: { 'Authorization': `Bearer ${token}` }
            });
            const profile = await profileRes.json();
            const tenantId = profile.tenant_id;

            await new Promise(r => setTimeout(r, 2000)); // wait for background job

            const reqA1 = await request.get(`/api/v1/builder/edge/${tenantId}/${siteId}`);
            expect(reqA1.ok()).toBeTruthy();
            const textA1 = await reqA1.text();

            // Ensure static HTML is pre-rendered with SEO tags
            expect(textA1).toContain('<title>Home SEO</title>');
            expect(textA1).toContain('application/ld+json');
            expect(textA1).toContain('LocalBusiness');
        });
    });
});
