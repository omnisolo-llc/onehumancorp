import { describe, it, expect, vi } from 'vitest';
import { GET } from './route';

describe('GET /api/v1/growth/milestone', () => {
  it('returns 400 if tenant_id is missing', async () => {
    const req = new Request('http://localhost/api/v1/growth/milestone');
    const res = await GET(req);
    expect(res.status).toBe(400);
  });

  it('returns backend data if tenant_id is provided', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({
        title: "100th Order Delivered! 🎉",
        subtitle: "You're growing fast. Share your success to unlock $50 in OHC credits.",
        shareText: "I just hit my 100th order using OHC to run my business! 🚀 Check them out and get $50 off your first month:",
        reward: "$50 Credit"
      }),
    });

    const req = new Request('http://localhost/api/v1/growth/milestone?tenant_id=my-store');
    const res = await GET(req);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.title).toBeDefined();
    expect(data.subtitle).toBeDefined();
    expect(data.shareText).toBeDefined();
    expect(data.reward).toBeDefined();
  });
});
