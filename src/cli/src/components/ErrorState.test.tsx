import React from 'react';
import { render } from 'ink-testing-library';
import { describe, it, expect, vi, afterEach } from 'vitest';
import { ErrorState } from './ErrorState.js';

describe('ErrorState', () => {
  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders correctly with error message', () => {
    const errorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    const { lastFrame } = render(<ErrorState error="Something went wrong!" />);

    expect(lastFrame()).toContain('ERROR:');
    expect(lastFrame()).toContain('Something went wrong!');
    expect(errorSpy).toHaveBeenCalledWith('Something went wrong!');
  });
});
