import React from 'react';
import { render } from 'ink-testing-library';
import { describe, it, expect } from 'vitest';
import { AgentStatus } from './AgentStatus.js';

describe('AgentStatus', () => {
  it('renders correctly', () => {
    const { lastFrame } = render(<AgentStatus status="Thinking..." />);
    expect(lastFrame()).toContain('Thinking...');
  });
});
