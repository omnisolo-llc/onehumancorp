import React from 'react';
import { render } from 'ink-testing-library';
import { MasterMenu } from './MasterMenu';
import { describe, it, expect, vi } from 'vitest';

describe('MasterMenu', () => {
  it('renders correctly', () => {
    const { lastFrame } = render(<MasterMenu />);
    expect(lastFrame()).toContain('Select an action');
    expect(lastFrame()).toContain('Run Developer Setup');
  });

  it('can navigate down', async () => {
    const { stdin, lastFrame } = render(<MasterMenu />);
    stdin.write('\x1B[B'); // down arrow
    await new Promise(r => setTimeout(r, 50));
    expect(lastFrame()).toContain('▶ 2) Configure Environment');
  });

  it('can navigate up', async () => {
    const { stdin, lastFrame } = render(<MasterMenu />);
    stdin.write('\x1B[B'); // down
    await new Promise(r => setTimeout(r, 50));
    stdin.write('\x1B[A'); // up
    await new Promise(r => setTimeout(r, 50));
    expect(lastFrame()).toContain('▶ 1) Run Developer Setup');
  });

  it('cannot navigate below bottom', async () => {
    const { stdin, lastFrame } = render(<MasterMenu />);
    for(let i=0; i<15; i++) {
        stdin.write('\x1B[B');
        await new Promise(r => setTimeout(r, 10));
    }
    expect(lastFrame()).toContain('▶ 0) Exit');
  });

  it('cannot navigate above top', async () => {
    const { stdin, lastFrame } = render(<MasterMenu />);
    stdin.write('\x1B[A');
    await new Promise(r => setTimeout(r, 50));
    expect(lastFrame()).toContain('▶ 1) Run Developer Setup');
  });

  it('can select an option', async () => {
    const consoleSpy = vi.spyOn(console, 'log').mockImplementation(() => {});
    const { stdin } = render(<MasterMenu />);
    stdin.write('\r'); // return
    await new Promise(r => setTimeout(r, 50));
    expect(consoleSpy).toHaveBeenCalledWith('Executing Run Developer Setup...');
    consoleSpy.mockRestore();
  });

  it('can select Exit', async () => {
    const exitSpy = vi.spyOn(process, 'exit').mockImplementation((() => {}) as any);
    const { stdin } = render(<MasterMenu />);
    for(let i=0; i<15; i++) {
        stdin.write('\x1B[B');
        await new Promise(r => setTimeout(r, 10));
    }
    stdin.write('\r'); // return
    await new Promise(r => setTimeout(r, 50));
    expect(exitSpy).toHaveBeenCalledWith(0);
    exitSpy.mockRestore();
  });

  it('ignores other keys', async () => {
    const { stdin, lastFrame } = render(<MasterMenu />);
    const initialFrame = lastFrame();
    stdin.write('a');
    await new Promise(r => setTimeout(r, 50));
    expect(lastFrame()).toBe(initialFrame);
  });
});
