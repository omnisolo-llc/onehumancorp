import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { POST } from './route';

describe('POST /api/v1/growth/promoter/generate', () => {
  let mockBackendUrl = 'http://mock-backend';

  beforeEach(() => {
    vi.stubEnv('BACKEND_URL', mockBackendUrl);
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.resetAllMocks();
  });

  it('returns generated posts via backend API', async () => {
    const mockResponse = {
      instagram: 'Backend IG',
      twitter: 'Backend TW',
      email: 'Backend EM'
    };

    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => mockResponse
    });

    const req = new Request('http://localhost/api/v1/growth/promoter/generate', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        product_name: 'Summer Dress',
        description: 'Perfect for the beach.',
        theme: 'summer',
        tenant: 'my-boutique'
      }),
    });

    const res = await POST(req);
    const json = await res.json();

    expect(res.status).toBe(200);
    expect(json).toEqual(mockResponse);
  });

  it('returns generated posts with viral branding when backend fails', async () => {
    (global.fetch as any).mockRejectedValueOnce(new TypeError('Failed to fetch'));

    const req = new Request('http://localhost/api/v1/growth/promoter/generate', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        product_name: 'Summer Dress',
        description: 'Perfect for the beach.',
        theme: 'summer',
        tenant: 'my-boutique'
      }),
    });

    const res = await POST(req);
    const json = await res.json();

    expect(res.status).toBe(200);
    expect(json.instagram).toContain('Summer Dress');
    expect(json.instagram).toContain('Perfect for the beach.');
    expect(json.instagram).toContain('summer special');
    expect(json.instagram).toContain('⚡ Powered by OHC');

    expect(json.twitter).toContain('Summer Dress');
    expect(json.twitter).toContain('/bio/my-boutique');
    expect(json.twitter).toContain('⚡ Powered by OHC');

    expect(json.email).toContain('Summer Dress');
    expect(json.email).toContain('/bio/my-boutique');
    expect(json.email).toContain('⚡ Powered by OHC');
  });
});
