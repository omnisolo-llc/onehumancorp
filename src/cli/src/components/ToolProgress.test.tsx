import React from 'react';
import { render } from 'ink-testing-library';
import { describe, it, expect } from 'vitest';
import { ToolProgress } from './ToolProgress';

describe('ToolProgress', () => {
  it('renders correctly', () => {
    const tools: { name: string; status: 'pending' | 'success' | 'error' }[] = [
      { name: 'ls -la', status: 'success' },
      { name: 'npm install', status: 'pending' },
      { name: 'npm test', status: 'error' }
    ];

    const { lastFrame } = render(<ToolProgress tools={tools} />);
    const output = lastFrame();

    expect(output).toContain('Tools Executed:');
    expect(output).toContain('[✓] ls -la');
    expect(output).toContain('[ ] npm install');
    expect(output).toContain('[x] npm test');
  });
});
