import { afterEach, describe, expect, it, vi } from 'vitest';
import { fetchJson, putJson } from './api';

afterEach(() => {
  vi.unstubAllGlobals();
});

describe('JSON API helpers', () => {
  it('parses successful JSON responses', async () => {
    const fetchMock = vi.fn().mockResolvedValue(
      new Response(JSON.stringify({ tenant: { base_currency: 'USD' } }), {
        status: 200,
        headers: { 'content-type': 'application/json' },
      }),
    );
    vi.stubGlobal('fetch', fetchMock);

    await expect(fetchJson('/api/v1/settings')).resolves.toEqual({
      tenant: { base_currency: 'USD' },
    });
    expect(fetchMock).toHaveBeenCalledWith('/api/v1/settings', {
      method: 'GET',
      headers: undefined,
      body: undefined,
      credentials: 'same-origin',
      cache: 'no-store',
    });
  });

  it('sends JSON PUT requests and surfaces backend errors', async () => {
    const fetchMock = vi.fn()
      .mockResolvedValueOnce(new Response(JSON.stringify({ success: true }), {
        status: 200,
        headers: { 'content-type': 'application/json' },
      }))
      .mockResolvedValueOnce(new Response(JSON.stringify({ error: 'not allowed' }), {
        status: 403,
        headers: { 'content-type': 'application/json' },
      }));
    vi.stubGlobal('fetch', fetchMock);

    await expect(putJson('/api/v1/settings', { base_currency: 'EUR' }))
      .resolves.toEqual({ success: true });
    await expect(putJson('/api/v1/settings', {})).rejects.toThrow('not allowed');
    expect(fetchMock.mock.calls[0]).toEqual([
      '/api/v1/settings',
      {
        method: 'PUT',
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify({ base_currency: 'EUR' }),
        credentials: 'same-origin',
        cache: 'no-store',
      },
    ]);
  });
});
