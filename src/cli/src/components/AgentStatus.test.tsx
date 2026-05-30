import React from 'react';
import { render } from 'ink-testing-library';
import { describe, it, expect } from 'vitest';
import { AgentStatus } from './AgentStatus';

describe('AgentStatus', () => {
  it('renders correctly with default loading state', () => {
    const { lastFrame } = render(<AgentStatus status="Thinking..." />);
    expect(lastFrame()).toContain('Thinking...');
  });

  it('renders correctly with success state', () => {
    const { lastFrame } = render(<AgentStatus status="Done" type="success" />);
    expect(lastFrame()).toContain('Done');
    expect(lastFrame()).toContain('✓');
  });

  it('renders correctly with error state', () => {
    const { lastFrame } = render(<AgentStatus status="Failed" type="error" />);
    expect(lastFrame()).toContain('Failed');
    expect(lastFrame()).toContain('✖');
  });
});
