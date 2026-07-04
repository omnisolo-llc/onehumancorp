import { GET } from './route';
import { NextRequest } from 'next/server';
import { describe, it, expect, vi } from 'vitest';

describe('Walkthrough API Route', () => {
  it('fetches data from backend successfully and maps fields', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve([{ selector: '#test-target', title: 'Test Step', text: 'Step content' }])
    });

    const req = new NextRequest('http://localhost:3000/api/walkthrough/test-page');
    const res = await GET(req, { params: { page: 'test-page' } });
    const data = await res.json();
    expect(data).toEqual([{ targetId: 'test-target', title: 'Test Step', content: 'Step content', position: 'bottom' }]);
  });

  it('handles backend error by returning 502', async () => {
    vi.spyOn(console, 'error').mockImplementation(() => {});
    global.fetch = vi.fn().mockRejectedValue(new Error('Network error'));

    const req = new NextRequest('http://localhost:3000/api/walkthrough/test-page');
    const res = await GET(req, { params: { page: 'test-page' } });
    const data = await res.json();
    expect(res.status).toBe(502);
    expect(data).toEqual({ error: 'Backend walkthrough service unavailable' });
  });
});
