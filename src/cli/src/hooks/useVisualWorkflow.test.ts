import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { renderHook, act } from '@testing-library/react';
import { useVisualWorkflow } from './useVisualWorkflow.js';

describe('useVisualWorkflow', () => {
  beforeEach(() => {
    vi.stubGlobal('fetch', vi.fn());
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it('should initialize with idle status', () => {
    const { result } = renderHook(() => useVisualWorkflow());
    expect(result.current.status).toBe('idle');
    expect(result.current.result).toBeNull();
    expect(result.current.error).toBeNull();
  });

  it('should handle successful workflow execution', async () => {
    const mockResponse = {
      ok: true,
      json: async () => ({
        result: { output: 'workflow success' },
      }),
    };
    (global.fetch as any).mockResolvedValue(mockResponse);

    const { result } = renderHook(() => useVisualWorkflow());

    await act(async () => {
      await result.current.runWorkflow({ nodes: [], edges: [] }, {});
    });

    expect(result.current.status).toBe('complete');
    expect(result.current.result).toBe('workflow success');
    expect(result.current.error).toBeNull();

    // Verify fetch was called with correct parameters
    const fetchCall = (global.fetch as any).mock.calls[0];
    expect(fetchCall[0]).toContain('/rpc');
    const body = JSON.parse(fetchCall[1].body);
    expect(body.method).toBe('execute_visual_workflow');
  });

  it('should handle successful workflow execution with no output', async () => {
    const mockResponse = {
      ok: true,
      json: async () => ({
        result: {},
      }),
    };
    (global.fetch as any).mockResolvedValue(mockResponse);

    const { result } = renderHook(() => useVisualWorkflow());

    await act(async () => {
      await result.current.runWorkflow({ nodes: [], edges: [] }, {});
    });

    expect(result.current.status).toBe('complete');
    expect(result.current.result).toBe('No output received.');
    expect(result.current.error).toBeNull();
  });

  it('should handle HTTP error', async () => {
    const mockResponse = {
      ok: false,
      status: 500,
    };
    (global.fetch as any).mockResolvedValue(mockResponse);

    const { result } = renderHook(() => useVisualWorkflow());

    await act(async () => {
      await result.current.runWorkflow({ nodes: [], edges: [] }, {});
    });

    expect(result.current.status).toBe('error');
    expect(result.current.error).toBe('HTTP error! status: 500');
  });

  it('should handle JSON-RPC error', async () => {
    const mockResponse = {
      ok: true,
      json: async () => ({
        error: { message: 'Workflow compilation failed' },
      }),
    };
    (global.fetch as any).mockResolvedValue(mockResponse);

    const { result } = renderHook(() => useVisualWorkflow());

    await act(async () => {
      await result.current.runWorkflow({ nodes: [], edges: [] }, {});
    });

    expect(result.current.status).toBe('error');
    expect(result.current.error).toBe('Workflow compilation failed');
  });

  it('should handle JSON-RPC error fallback message', async () => {
    const mockResponse = {
      ok: true,
      json: async () => ({
        error: {},
      }),
    };
    (global.fetch as any).mockResolvedValue(mockResponse);

    const { result } = renderHook(() => useVisualWorkflow());

    await act(async () => {
      await result.current.runWorkflow({ nodes: [], edges: [] }, {});
    });

    expect(result.current.status).toBe('error');
    expect(result.current.error).toBe('JSON-RPC Error');
  });

  it('should handle arbitrary error', async () => {
    (global.fetch as any).mockRejectedValue(new Error('Network disconnected'));

    const { result } = renderHook(() => useVisualWorkflow());

    await act(async () => {
      await result.current.runWorkflow({ nodes: [], edges: [] }, {});
    });

    expect(result.current.status).toBe('error');
    expect(result.current.error).toBe('Network disconnected');
  });

  it('should handle arbitrary error with fallback message', async () => {
    (global.fetch as any).mockRejectedValue({});

    const { result } = renderHook(() => useVisualWorkflow());

    await act(async () => {
      await result.current.runWorkflow({ nodes: [], edges: [] }, {});
    });

    expect(result.current.status).toBe('error');
    expect(result.current.error).toBe('An error occurred during execution.');
  });
});
