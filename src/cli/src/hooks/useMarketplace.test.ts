import { expect, test, describe, vi, beforeEach } from 'vitest';
import { renderHook, waitFor } from '@testing-library/react';
import { useMarketplace } from './useMarketplace.js';

describe('useMarketplace', () => {
  beforeEach(() => {
    global.fetch = vi.fn() as any;
  });

  test('fetches and returns agents successfully', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        jsonrpc: '2.0',
        result: [{ id: '1', name: 'Agent 1', description: 'Desc 1', author: 'Author 1', downloads: 100 }],
        id: 'test'
      })
    });

    const { result } = renderHook(() => useMarketplace());

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    expect(result.current.agents).toHaveLength(1);
    expect(result.current.agents[0].name).toBe('Agent 1');
    expect(result.current.error).toBeNull();
  });

  test('handles fetch errors', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: false,
      status: 500
    });

    const { result } = renderHook(() => useMarketplace());

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    expect(result.current.error).toContain('HTTP error');
    expect(result.current.agents).toHaveLength(0);
  });
});

  test('handles unknown error format', async () => {
    (global.fetch as any).mockRejectedValueOnce('Network disconnected');

    const { result } = renderHook(() => useMarketplace());

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    expect(result.current.error).toContain('Failed to fetch marketplace agents.');
  });
