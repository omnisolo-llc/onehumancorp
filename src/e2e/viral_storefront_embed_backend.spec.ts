import { test, expect } from '@playwright/test';

test.describe('Viral Storefront Embed Backend API', () => {
  test('returns glassmorphism styled widget with powered by ohc link', async ({ request }) => {
    const response = await request.get('/api/v1/growth/storefront/embed?tenant=my_viral_store&product_name=TestProd&price=19.99');
    expect(response.ok()).toBeTruthy();

    const html = await response.text();

    // Verify glassmorphism style presence
    expect(html).toContain('backdrop-filter: blur(20px) saturate(200%)');

    // Verify the title and price are injected
    expect(html).toContain('TestProd');
    expect(html).toContain('19.99');

    // Verify the viral acquisition loop is present
    expect(html).toContain('href="ohc://join?ref=my_viral_store"');
    expect(html).toContain('Powered by OHC');
  });

  test('handles dark mode correctly', async ({ request }) => {
    const response = await request.get('/api/v1/growth/storefront/embed?tenant=test&theme=dark');
    expect(response.ok()).toBeTruthy();

    const html = await response.text();

    // Verify dark mode class is applied to body
    expect(html).toContain('<body class="dark-mode">');
  });
});
