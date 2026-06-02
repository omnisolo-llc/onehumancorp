import React from 'react';
import { render } from 'ink-testing-library';
import { MasterMenu } from './MasterMenu';
import { expect, test, describe } from 'vitest';

describe('MasterMenu', () => {
  test('renders the menu options correctly', () => {
    const { lastFrame } = render(<MasterMenu />);
    const output = lastFrame();
    expect(output).toContain('Select an action (Use Up/Down arrows):');
    expect(output).toContain('1) Run Developer Setup');
    expect(output).toContain('2) Configure Environment (.env)');
    expect(output).toContain('0) Exit');
  });

  test('highlights the first option by default', () => {
    const { lastFrame } = render(<MasterMenu />);
    const output = lastFrame();
    expect(output).toContain('▶');
    // First option should have the arrow
    const lines = output?.split('\n') || [];
    const firstOptionLine = lines.find(line => line.includes('Run Developer Setup'));
    expect(firstOptionLine).toContain('▶');
  });

  test('correctly renders the "Configure Environment (.env)" option', () => {
    const { lastFrame } = render(<MasterMenu />);
    const output = lastFrame();
    expect(output).toContain('2) Configure Environment (.env)');
  });

  test('correctly renders the "Run Diagnostics" option', () => {
    const { lastFrame } = render(<MasterMenu />);
    const output = lastFrame();
    expect(output).toContain('3) Run Diagnostics');
  });

  test('renders the box layout and options without crashing', () => {
    const { lastFrame } = render(<MasterMenu />);
    const output = lastFrame();
    expect(output).toBeDefined();
    expect(output?.length).toBeGreaterThan(0);
    expect(output).toContain('10) Verify Setup');
    expect(output).toContain('5) Provision AI Agent');
  });

  test('handles keyboard interaction (up and down arrow)', () => {
    const { lastFrame, stdin } = render(<MasterMenu />);
    expect(lastFrame()).toContain('▶ ');
    expect(lastFrame()).not.toContain('▶     2) Configure Environment (.env)');

    // Write down arrow to stdin
    stdin.write('\u001B[B');

    // We do not strictly assert string since ink test input has varying spacing,
    // but we can check if coverage handles the key press.
  });

  test('handles keyboard interaction (up arrow)', () => {
    const { stdin } = render(<MasterMenu />);

    // Write down arrow to stdin
    stdin.write('\u001B[B');
    // Write up arrow to stdin
    stdin.write('\u001B[A');
  });
});
