import { describe, it, expect, vi, beforeEach } from 'vitest';
import { renderHook, act } from '@testing-library/react';
import { useVisualWorkflow } from './useVisualWorkflow.js';

describe('useVisualWorkflow', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    global.fetch = vi.fn();
  });

  it('initializes with default state', () => {
    const { result } = renderHook(() => useVisualWorkflow());
    expect(result.current.loading).toBe(false);
    expect(result.current.error).toBeNull();
    expect(result.current.result).toBeNull();
  });

  it('runs workflow successfully', async () => {
    const mockResponse = { result: { output: 'Success' }, error: null };
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => mockResponse,
    });

    const { result } = renderHook(() => useVisualWorkflow());
    await act(async () => {
      await result.current.runWorkflow({}, {});
    });

    expect(result.current.loading).toBe(false);
    expect(result.current.error).toBeNull();
    expect(result.current.result).toBe('Success');
  });

  it('handles fetch error', async () => {
    (global.fetch as any).mockRejectedValueOnce(new Error('Network error'));

    const { result } = renderHook(() => useVisualWorkflow());
    await act(async () => {
      await result.current.runWorkflow({}, {});
    });

    expect(result.current.loading).toBe(false);
    expect(result.current.error).toBe('Network error');
  });
});
