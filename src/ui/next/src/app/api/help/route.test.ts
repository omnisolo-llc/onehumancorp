import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { GET } from './route';
import { NextRequest } from 'next/server';

describe('/api/help GET', () => {
  beforeEach(() => {
    global.fetch = vi.fn().mockImplementation((url) => {
      if (url.includes('/api/help')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve([
            { title: "Welcome to One Human Corp", desc: "Welcome to One Human Corp! This is a simple app that helps you manage your small business.", link: "/help/getting-started-1" },
            { title: "Setting up your storefront", desc: "To set up your storefront, go to the 'My Store' tab", link: "/help/my-store-1" }
          ])
        });
      }
      return Promise.resolve({ ok: false, status: 404 });
    });
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('returns the help articles list with specific content', async () => {
    const req = new NextRequest('http://localhost/api/help');
    const response = await GET(req);
    expect(response.status).toBe(200);
    const data = await response.json();

    expect(Array.isArray(data)).toBe(true);
    expect(data.length).toBe(2);

    // Check specific items to ensure content is correct
    const gettingStarted = data.find((item: any) => item.link === '/help/getting-started-1');
    expect(gettingStarted).toBeDefined();
    expect(gettingStarted.title).toBe('Welcome to One Human Corp');
    expect(gettingStarted.desc).toContain('manage your small business');

    const myStore = data.find((item: any) => item.link === '/help/my-store-1');
    expect(myStore).toBeDefined();
    expect(myStore.title).toBe('Setting up your storefront');
    expect(myStore.desc).toContain('go to the \'My Store\' tab');

    // Ensure all items have the required fields
    data.forEach((item: any) => {
      expect(item).toHaveProperty('title');
      expect(typeof item.title).toBe('string');
      expect(item).toHaveProperty('desc');
      expect(typeof item.desc).toBe('string');
      expect(item).toHaveProperty('link');
      expect(typeof item.link).toBe('string');
    });
  });
});
