import React from 'react';
import { render } from 'ink-testing-library';
import { MasterMenu } from './MasterMenu';
import { expect, test, describe, vi } from 'vitest';

describe('MasterMenu', () => {
  test('renders the menu options correctly', () => {
    const { lastFrame } = render(<MasterMenu />);
    const output = lastFrame();
    expect(output).toContain('Select an action (Use Up/Down arrows):');
    expect(output).toContain('1) Run Developer Setup');
    expect(output).toContain('2) Configure Environment (.env)');
    expect(output).toContain('View Agent Protocol Tasks');
    expect(output).toContain('0) Exit');
  });

  test('handles view protocol toggle', async () => {
    const { stdin, lastFrame } = render(<MasterMenu />);

    // Scroll to View Agent Protocol Tasks (index 10)
    for (let i = 0; i < 10; i++) {
        stdin.write('\u001B[B');
        await new Promise(r => setTimeout(r, 20));
    }

    // Press Enter to open
    stdin.write('\r');
    await new Promise(r => setTimeout(r, 20));

    expect(lastFrame()).toContain('Agent Protocol Tasks');

    // Press Esc to go back
    stdin.write('\u001B'); // Esc
    await new Promise(r => setTimeout(r, 20));

    expect(lastFrame()).toContain('Select an action');
  });

  test('highlights the first option by default', () => {
    const { lastFrame } = render(<MasterMenu />);
    const output = lastFrame();
    expect(output).toContain('▶');
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

  test('handles keyboard interaction (down arrow)', async () => {
    const { stdin, lastFrame } = render(<MasterMenu />);

    // Write down arrow to stdin
    stdin.write('\u001B[B'); // Down Arrow

    // allow event loop to process
    await new Promise(r => setTimeout(r, 20));

    // Just checking it handles it without crashing is what was tested originally
    expect(lastFrame()).toBeDefined();
  });

  test('handles keyboard interaction (up arrow)', async () => {
    const { stdin, lastFrame } = render(<MasterMenu />);
    stdin.write('\u001B[B'); // Down Arrow

    await new Promise(r => setTimeout(r, 20));

    stdin.write('\u001B[A'); // Up Arrow
    await new Promise(r => setTimeout(r, 20));

    expect(lastFrame()).toBeDefined();
  });

  test('handles keyboard interaction (return)', () => {
    const logSpy = vi.spyOn(console, 'info').mockImplementation(() => {});
    const { stdin } = render(<MasterMenu />);
    stdin.write('\r');
    expect(logSpy).toHaveBeenCalledWith('Executing Run Developer Setup...');
    logSpy.mockRestore();
  });

  test('handles keyboard interaction (exit option)', async () => {
    const exitSpy = vi.spyOn(process, 'exit').mockImplementation((() => {}) as any);
    const { stdin, lastFrame } = render(<MasterMenu />);

    // Exit is the last option.
    // MasterMenu has 11 options. index 10 is Exit.
    for (let i = 0; i < 15; i++) {
        stdin.write('\u001B[B');
        // Let React process the event
        await new Promise(r => setTimeout(r, 20));
    }

    stdin.write('\r');
    await new Promise(r => setTimeout(r, 20));

    expect(exitSpy).toHaveBeenCalledWith(0);
    exitSpy.mockRestore();
  });
});
