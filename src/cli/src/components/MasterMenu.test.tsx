import React from 'react';
import { render } from 'ink-testing-library';
import { MasterMenu } from './MasterMenu.js';
import { describe, it, expect, vi } from 'vitest';

const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

describe('MasterMenu', () => {
  it('renders the menu options correctly', () => {
    const { lastFrame } = render(<MasterMenu />);
    const output = lastFrame();
    expect(output).toContain('Select an action');
    expect(output).toContain('1) Run Developer Setup');
    expect(output).toContain('0) Exit');
  });

  it('highlights the first option by default', () => {
    const { lastFrame } = render(<MasterMenu />);
    const output = lastFrame();
    expect(output).toContain('▶ 1) Run Developer Setup');
  });

  it('correctly renders the "Configure Environment (.env)" option', () => {
     const { lastFrame } = render(<MasterMenu />);
     const output = lastFrame();
     expect(output).toContain('2) Configure Environment (.env)');
  });

  it('correctly renders the "Run Diagnostics" option', () => {
     const { lastFrame } = render(<MasterMenu />);
     const output = lastFrame();
     expect(output).toContain('3) Run Diagnostics');
  });

  it('renders the box layout and options without crashing', () => {
    const { lastFrame } = render(<MasterMenu />);
    const output = lastFrame();
    expect(output).toBeDefined();
    expect(output?.length).toBeGreaterThan(0);
    expect(output).toContain('12) Verify Setup');
    expect(output).toContain('5) Provision AI Agent');
  });

  it('handles keyboard interaction (down arrow)', async () => {
    const onSelect = vi.fn();
    const { stdin, lastFrame } = render(<MasterMenu onSelect={onSelect} />);

    stdin.write('\x1B[B'); // Down arrow
    await delay(10);
    expect(lastFrame()).toContain('▶ 2) Configure Environment (.env)');
  });

  it('handles keyboard interaction (up arrow)', async () => {
    const onSelect = vi.fn();
    const { stdin, lastFrame } = render(<MasterMenu onSelect={onSelect} />);

    stdin.write('\x1B[B'); // Down arrow
    await delay(10);
    stdin.write('\x1B[A'); // Up arrow
    await delay(10);

    expect(lastFrame()).toContain('▶ 1) Run Developer Setup');
  });

  it('handles keyboard interaction (return)', async () => {
    const onSelect = vi.fn();
    const { stdin } = render(<MasterMenu onSelect={onSelect} />);

    stdin.write('\r'); // Return
    await delay(10);

    expect(onSelect).toHaveBeenCalledWith('Run Developer Setup');
  });

  it('handles keyboard interaction (exit option)', async () => {
    const originalExit = process.exit;
    let exitCode: number | undefined;
    (process as any).exit = (code: number) => {
      exitCode = code;
    };

    const { stdin } = render(<MasterMenu />);

    // Press down arrow enough times to reach "Exit"
    for (let i = 0; i < 15; i++) {
        stdin.write('\x1B[B');
        await delay(10);
    }

    stdin.write('\r');
    await delay(10);

    expect(exitCode).toBe(0);

    process.exit = originalExit;
  });
});
