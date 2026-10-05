import { act, renderHook } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';

import { useAuthenticatedPolling } from './useAuthenticatedPolling';

describe('useAuthenticatedPolling', () => {
  afterEach(() => {
    vi.restoreAllMocks();
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

  it('waits for the current poll to settle before starting another', async () => {
    vi.useFakeTimers();
    let finish!: () => void;
    const onPoll = vi.fn(() => new Promise<void>(resolve => { finish = resolve; }));
    const { unmount } = renderHook(() => useAuthenticatedPolling({ onPoll, intervalMs: 1_000 }));
    await act(async () => { vi.advanceTimersByTime(3_000); });
    expect(onPoll).toHaveBeenCalledTimes(1);
    await act(async () => { finish(); });
    await act(async () => { vi.advanceTimersByTime(1_000); });
    expect(onPoll).toHaveBeenCalledTimes(2);
    unmount();
    await act(async () => { finish(); });
  });

  it.each(['unmount', 'disable', 'interval change'])(
    'aborts the active read on %s', async change => {
      vi.useFakeTimers();
      const signals: AbortSignal[] = [];
      let finish!: () => void;
      const onPoll = vi.fn((signal: AbortSignal) => {
        signals.push(signal);
        return new Promise<void>(resolve => { finish = resolve; });
      });
      const { rerender, unmount } = renderHook(
        ({ enabled, intervalMs }) => useAuthenticatedPolling({ onPoll, enabled, intervalMs }),
        { initialProps: { enabled: true, intervalMs: 1_000 } },
      );
      await act(async () => { vi.advanceTimersByTime(1_000); });
      expect(signals[0]).toBeInstanceOf(AbortSignal);
      if (change === 'unmount') unmount();
      else rerender({ enabled: change !== 'disable', intervalMs: change === 'interval change' ? 500 : 1_000 });
      expect(signals[0].aborted).toBe(true);
      await act(async () => { vi.advanceTimersByTime(5_000); });
      expect(onPoll).toHaveBeenCalledTimes(1);
      await act(async () => { finish(); });
      unmount();
    },
  );

  it('contains synchronous callback failures and polls again', async () => {
    vi.useFakeTimers();
    const failure = new Error('read failed');
    const onPoll = vi.fn(() => { throw failure; });
    const report = vi.spyOn(console, 'error').mockImplementation(() => undefined);
    const { unmount } = renderHook(() => useAuthenticatedPolling({ onPoll, intervalMs: 1_000 }));
    await act(async () => { vi.advanceTimersByTime(1_000); });
    expect(report).toHaveBeenCalledWith('Authenticated feed polling failed:', failure);
    await act(async () => { vi.advanceTimersByTime(1_000); });
    expect(onPoll).toHaveBeenCalledTimes(2);
    unmount();
  });

  it('uses the current callback after rerender', async () => {
    vi.useFakeTimers();
    const first = vi.fn(), second = vi.fn();
    const { rerender, unmount } = renderHook(
      ({ onPoll }) => useAuthenticatedPolling({ onPoll, intervalMs: 1_000 }),
      { initialProps: { onPoll: first } },
    );
    rerender({ onPoll: second });
    await act(async () => { vi.advanceTimersByTime(1_000); });
    expect(first).not.toHaveBeenCalled();
    expect(second).toHaveBeenCalledTimes(1);
    unmount();
  });

});
