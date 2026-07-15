import { GET } from './route';
import { NextRequest } from 'next/server';

describe('Footer Branding Embed API', () => {
  it('returns javascript code with default parameters', async () => {
    const req = new NextRequest('http://localhost/api/v1/growth/footer-branding/embed');
    const res = await GET(req);

    expect(res.status).toBe(200);
    expect(res.headers.get('Content-Type')).toBe('application/javascript');

    const body = await res.text();
    expect(body).toContain("const tenant = 'default'");
    expect(body).toContain("const style = 'pill'");
    expect(body).toContain("const text = 'Powered by OHC'");
    expect(body).toContain("const theme = 'light'");
    expect(body).toContain('https://app.onehumancorp.com/invite/default');
  });

  it('returns javascript code with custom parameters', async () => {
    const req = new NextRequest('http://localhost/api/v1/growth/footer-branding/embed?tenant=test-tenant&style=footer&text=Made%20with%20Love&theme=dark');
    const res = await GET(req);

    expect(res.status).toBe(200);

    const body = await res.text();
    expect(body).toContain("const tenant = 'test-tenant'");
    expect(body).toContain("const style = 'footer'");
    expect(body).toContain("const text = 'Made with Love'");
    expect(body).toContain("const theme = 'dark'");
    expect(body).toContain('https://app.onehumancorp.com/invite/test-tenant');
  });
});
