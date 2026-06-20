import { describe, it, expect } from 'vitest';
import { GET } from './route';

describe('Storefront Embed Widget', () => {
  it('returns 400 if tenant is missing', async () => {
    const req = new Request('http://localhost/api/v1/growth/storefront/embed');
    const res = await GET(req);
    expect(res.status).toBe(400);
  });

  it('returns HTML with correct tenant and default theme', async () => {
    const req = new Request('http://localhost/api/v1/growth/storefront/embed?tenant=maya-cakes');
    const res = await GET(req);
    expect(res.status).toBe(200);

    const text = await res.text();
    expect(text).toContain('<!DOCTYPE html>');
    expect(text).toContain('https://ohc.app?ref=maya-cakes');
  });
});
