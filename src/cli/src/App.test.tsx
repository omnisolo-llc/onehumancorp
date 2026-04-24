import React from 'react';
import { render } from 'ink-testing-library';
import { describe, it, expect, vi } from 'vitest';
import { App } from './App';

describe('App', () => {
  it('renders correctly and tests useEffect coverage including cleanup', async () => {
    vi.useFakeTimers();

    const { lastFrame, unmount } = render(<App />);
    let output = lastFrame();

    expect(output).toContain('ONE HUMAN CORP');
    expect(output).toContain('Initializing Agent...');
    expect(output).toContain('Tools Executed:');
    expect(output).toContain('OHC Interactive Harness');

    // Fast-forward timers to trigger the useEffect state change
    await vi.advanceTimersByTimeAsync(2000);

    output = lastFrame();
    expect(output).toContain('Analyzing Codebase...');

    // Unmount to trigger the cleanup function and achieve 100% coverage
    unmount();

    vi.useRealTimers();
  });
});
