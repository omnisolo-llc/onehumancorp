import { POST } from './route';
import { describe, it, expect } from 'vitest';

describe('Email Signature Generator API', () => {
  it('generates html with branding by default', async () => {
    const req = new Request('http://localhost/api/v1/growth/email-signature/generate', {
      method: 'POST',
      body: JSON.stringify({
        name: 'Test Name',
        role: 'Tester',
        company: 'TestCo',
        tenantId: 'test-tenant',
        hasPro: false,
        removeBranding: false,
      }),
    });

    const res = await POST(req);
    const json = await res.json();

    expect(json.html).toContain('Test Name');
    expect(json.html).toContain('Tester | TestCo');
    expect(json.html).toContain('⚡ Powered by OHC');
    expect(json.html).toContain('ref=test-tenant');
  });

  it('removes branding if requested and hasPro is true', async () => {
    const req = new Request('http://localhost/api/v1/growth/email-signature/generate', {
      method: 'POST',
      body: JSON.stringify({
        name: 'Pro User',
        role: 'CEO',
        company: 'ProCo',
        tenantId: 'pro-tenant',
        hasPro: true,
        removeBranding: true,
      }),
    });

    const res = await POST(req);
    const json = await res.json();

    expect(json.html).toContain('Pro User');
    expect(json.html).not.toContain('⚡ Powered by OHC');
  });

  it('includes optional fields', async () => {
    const req = new Request('http://localhost/api/v1/growth/email-signature/generate', {
      method: 'POST',
      body: JSON.stringify({
        phone: '123-456-7890',
        website: 'example.com',
        bannerText: 'Buy my stuff',
      }),
    });

    const res = await POST(req);
    const json = await res.json();

    expect(json.html).toContain('123-456-7890');
    expect(json.html).toContain('https://example.com');
    expect(json.html).toContain('Buy my stuff ✨');
  });
});
