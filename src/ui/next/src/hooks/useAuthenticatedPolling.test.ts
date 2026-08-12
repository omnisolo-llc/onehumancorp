import { act, renderHook } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';

import { useAuthenticatedPolling } from './useAuthenticatedPolling';

describe('useAuthenticatedPolling', () => {
  afterEach(() => {
    vi.useRealTimers();
  });

  it('polls on the configured interval and stops on unmount', () => {
    vi.useFakeTimers();
    const onPoll = vi.fn();
    const { unmount } = renderHook(() =>
      useAuthenticatedPolling({ onPoll, intervalMs: 1_000 }),
    );

    act(() => {
      vi.advanceTimersByTime(999);
    });
    expect(onPoll).not.toHaveBeenCalled();

    act(() => {
      vi.advanceTimersByTime(1);
    });
    expect(onPoll).toHaveBeenCalledTimes(1);

    unmount();
    act(() => {
      vi.advanceTimersByTime(2_000);
    });
    expect(onPoll).toHaveBeenCalledTimes(1);
  });
});
