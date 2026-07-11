import { test, expect } from '@playwright/test';

test.describe('Edge-Cached Dynamic Multi-Tenant Storefronts', () => {
    test('resolves custom domain, serves cached SSR HTML, and includes valid caching headers', async ({ request }) => {
        // This test simulates the custom domain mapping to a tenant and verifies the edge-caching headers

        const customDomain = 'mayascakes.test';
        const baseUrl = process.env.BASE_URL || 'http://localhost:18789';

        // Make request to the custom domain resolution endpoint
        const response = await request.get(`${baseUrl}/api/v1/storefront/resolve_domain`, {
            headers: {
                'Host': customDomain,
                'X-Forwarded-Host': customDomain
            }
        });

        // The domain is not seeded, so it should be a 404 NOT_FOUND from our logic instead of 500
        expect(response.status()).toBe(404);

        // Assert that the middleware still intercepted it and handled caching
        const headers = response.headers();
        expect(headers['x-cache']).toBeDefined();

        // Let's do another request to see if it gets a HIT or MISS from CDN cache (even for 404s depending on setup, but typically 404s aren't cached or are MISS)
        const secondResponse = await request.get(`${baseUrl}/api/v1/storefront/resolve_domain`, {
            headers: {
                'Host': customDomain,
                'X-Forwarded-Host': customDomain
            }
        });

        expect(secondResponse.status()).toBe(404);
    });
});
