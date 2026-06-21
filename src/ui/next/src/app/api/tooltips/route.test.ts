import { GET } from './route';
import { NextRequest } from 'next/server';
import { describe, it, expect, vi } from 'vitest';

describe('Tooltips API', () => {
  it('returns fallback tooltips when backend fails', async () => {
    global.fetch = vi.fn().mockRejectedValue(new Error('Network error'));
    const req = new NextRequest('http://localhost:3000/api/tooltips');
    const res = await GET(req);
    const data = await res.json();
    expect(data['changelog-nav-tooltip']).toBeDefined();
  });
});
