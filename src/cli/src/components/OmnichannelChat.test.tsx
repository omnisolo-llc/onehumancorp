
import React from 'react';
import { render } from 'ink-testing-library';
import { describe, it, expect, vi } from 'vitest';
import { OmnichannelChat } from './OmnichannelChat';

describe('OmnichannelChat', () => {
  it('renders correctly', () => {
    const { lastFrame } = render(<OmnichannelChat />);
    expect(lastFrame()!).toContain('Omnichannel Chat Testing');
    expect(lastFrame()!).toContain('Copilot Mode:');
  });
});
