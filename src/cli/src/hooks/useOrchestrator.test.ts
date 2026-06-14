import { renderHook, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { useOrchestrator } from './useOrchestrator';

const mockFetch = vi.fn();
global.fetch = mockFetch;

describe('useOrchestrator', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should initialize state', () => {
    const { result, unmount } = renderHook(() => useOrchestrator());

    expect(result.current.status).toBe('Idle');
    expect(result.current.tools.length).toBe(0);
    expect(result.current.error).toBeNull();
    expect(result.current.output).toBeNull();

    unmount();
  });

  it('should call runAgent successfully', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        result: { output: 'Hello from agent' },
      }),
    });

    const { result, unmount } = renderHook(() => useOrchestrator());

    await act(async () => {
      await result.current.runAgent('Hello');
    });

    expect(mockFetch).toHaveBeenCalledTimes(1);
    expect(result.current.status).toBe('Complete');
    expect(result.current.output).toBe('Hello from agent');
    expect(result.current.error).toBeNull();

    unmount();
  });

  it('should call runAgent successfully without output', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        result: {},
      }),
    });

    const { result, unmount } = renderHook(() => useOrchestrator());

    await act(async () => {
      await result.current.runAgent('Hello');
    });

    expect(mockFetch).toHaveBeenCalledTimes(1);
    expect(result.current.status).toBe('Complete');
    expect(result.current.output).toBe('No output received.');
    expect(result.current.error).toBeNull();

    unmount();
  });

  it('should handle runAgent error', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: false,
      status: 500,
      json: async () => ({}),
    });

    const { result, unmount } = renderHook(() => useOrchestrator());

    await act(async () => {
      await result.current.runAgent('Hello');
    });

    expect(mockFetch).toHaveBeenCalledTimes(1);
    expect(result.current.status).toBe('Error');
    expect(result.current.output).toBeNull();
    expect(result.current.error).toBe('HTTP error! status: 500');

    unmount();
  });

  it('should handle JSON-RPC error response', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        error: { message: 'JSON-RPC Custom Error' },
      }),
    });

    const { result, unmount } = renderHook(() => useOrchestrator());

    await act(async () => {
      await result.current.runAgent('Hello');
    });

    expect(mockFetch).toHaveBeenCalledTimes(1);
    expect(result.current.status).toBe('Error');
    expect(result.current.output).toBeNull();
    expect(result.current.error).toBe('JSON-RPC Custom Error');

    unmount();
  });

  it('should handle JSON-RPC default error response', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        error: {},
      }),
    });

    const { result, unmount } = renderHook(() => useOrchestrator());

    await act(async () => {
      await result.current.runAgent('Hello');
    });

    expect(mockFetch).toHaveBeenCalledTimes(1);
    expect(result.current.status).toBe('Error');
    expect(result.current.output).toBeNull();
    expect(result.current.error).toBe('JSON-RPC Error');

    unmount();
  });

  it('should handle unexpected generic error', async () => {
    mockFetch.mockRejectedValueOnce(new Error('Network disconnected'));

    const { result, unmount } = renderHook(() => useOrchestrator());

    await act(async () => {
      await result.current.runAgent('Hello');
    });

    expect(mockFetch).toHaveBeenCalledTimes(1);
    expect(result.current.status).toBe('Error');
    expect(result.current.output).toBeNull();
    expect(result.current.error).toBe('Network disconnected');

    unmount();
  });

  it('should handle unexpected generic error without message', async () => {
    mockFetch.mockRejectedValueOnce({});

    const { result, unmount } = renderHook(() => useOrchestrator());

    await act(async () => {
      await result.current.runAgent('Hello');
    });

    expect(mockFetch).toHaveBeenCalledTimes(1);
    expect(result.current.status).toBe('Error');
    expect(result.current.output).toBeNull();
    expect(result.current.error).toBe('An error occurred during execution.');

    unmount();
  });
});
