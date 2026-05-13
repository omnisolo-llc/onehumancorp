import React from 'react';
import { render } from 'ink-testing-library';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { App } from './App';
import * as orchestrator from './hooks/useOrchestrator';

vi.mock('./components/Wizard', () => ({
  Wizard: ({ onComplete }: { onComplete: (state: any) => void }) => {
    React.useEffect(() => {
      onComplete({
        businessType: 'Type',
        companyName: 'Name',
        sellingCategories: 'Cat',
        productName: 'Prod',
        productPrice: '1',
        paymentPref: 'Pref',
        template: 'Temp',
        domain: 'Dom',
        adminName: 'Admin',
        adminEmail: 'Email'
      });
    }, [onComplete]);
    return null;
  }
}));

describe('App', () => {
  beforeEach(() => {
    global.fetch = vi.fn().mockResolvedValue({ ok: true });
  });

  it('renders correctly', async () => {
    vi.spyOn(orchestrator, 'useOrchestrator').mockReturnValue({
      status: 'ok',
      tools: [],
      error: null
    });

    const { lastFrame, unmount, stdin } = render(<App />);

    // Wait for the mock wizard to fire
    await new Promise(r => setTimeout(r, 50));

    let output = lastFrame();
    expect(output).toContain('ONE HUMAN CORP');
    expect(output).toContain('Standalone Agent Mode');
    expect(global.fetch).toHaveBeenCalled();

    // Simulate user input for coverage
    stdin.write('test prompt\r');
    await new Promise(r => setTimeout(r, 50));

    output = lastFrame();
    expect(output).toContain('test prompt');

    unmount();
  });

  it('renders error state correctly when error occurs', async () => {
    global.fetch = vi.fn().mockRejectedValue(new Error('Network error'));

    vi.spyOn(orchestrator, 'useOrchestrator').mockReturnValue({
      status: 'error',
      tools: [],
      error: 'Test Error'
    });

    const { lastFrame, unmount } = render(<App />);

    await new Promise(r => setTimeout(r, 50));

    expect(lastFrame()).toContain('Test Error');
    expect(global.fetch).toHaveBeenCalled();

    unmount();
  });
});
