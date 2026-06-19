import React from 'react';
import { render } from 'ink-testing-library';
import { describe, it, expect } from 'vitest';
import { ErrorState } from './ErrorState.js';

describe('ErrorState', () => {
  it('renders correctly with error message', () => {
    const { lastFrame } = render(<ErrorState error="Something went wrong!" />);
    expect(lastFrame()).toContain('ERROR:');
    expect(lastFrame()).toContain('Something went wrong!');
  });
});
