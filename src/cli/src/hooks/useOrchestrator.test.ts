import { renderHook, act } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { useOrchestrator } from './useOrchestrator';

describe('useOrchestrator', () => {
  it('should initialize and update state', () => {
    vi.useFakeTimers();
    const { result, unmount } = renderHook(() => useOrchestrator());

    expect(result.current.status).toBe('Initializing Agent...');
    expect(result.current.tools[1].status).toBe('pending');

    act(() => {
      vi.advanceTimersByTime(2000);
    });

    expect(result.current.status).toBe('Analyzing Codebase...');
    expect(result.current.tools[1].status).toBe('success');

    unmount();
    vi.useRealTimers();
  });
});
