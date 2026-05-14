import React from 'react';
import { render } from 'ink-testing-library';
import { describe, it, expect } from 'vitest';
import { HelpCenter } from './HelpCenter';

describe('HelpCenter', () => {
  it('renders the collapsed help prompt by default', () => {
    const { lastFrame } = render(<HelpCenter />);
    expect(lastFrame()?.includes('Need help?')).toBe(true);
  });
});
