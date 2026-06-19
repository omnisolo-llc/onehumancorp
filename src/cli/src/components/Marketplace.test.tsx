import React from 'react';
import { render } from 'ink-testing-library';
import { Marketplace } from './Marketplace.js';
import { expect, test, describe, vi, beforeEach } from 'vitest';
import * as useMarketplaceModule from '../hooks/useMarketplace.js';

describe('Marketplace', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  test('renders loading state', () => {
    vi.spyOn(useMarketplaceModule, 'useMarketplace').mockReturnValue({
      agents: [],
      loading: true,
      error: null,
      fetchAgents: vi.fn()
    });

    const { lastFrame } = render(<Marketplace onBack={() => {}} />);
    const output = lastFrame();
    expect(output).toContain('Fetching Pre-built Agents...');
  });

  test('renders error state', () => {
    vi.spyOn(useMarketplaceModule, 'useMarketplace').mockReturnValue({
      agents: [],
      loading: false,
      error: 'Network Error',
      fetchAgents: vi.fn()
    });

    const { lastFrame } = render(<Marketplace onBack={() => {}} />);
    const output = lastFrame();
    expect(output).toContain('Error: Network Error');
  });

  test('renders agents and handles selection', () => {
    vi.spyOn(useMarketplaceModule, 'useMarketplace').mockReturnValue({
      agents: [
        { id: '1', name: 'Agent 1', description: 'Desc 1', author: 'Author 1', downloads: 100 },
        { id: '2', name: 'Agent 2', description: 'Desc 2', author: 'Author 2', downloads: 200 }
      ],
      loading: false,
      error: null,
      fetchAgents: vi.fn()
    });

    const { lastFrame } = render(<Marketplace onBack={() => {}} />);
    const output = lastFrame();
    expect(output).toContain('Agent 1');
    expect(output).toContain('Author 1');
    expect(output).toContain('Desc 1');
    expect(output).toContain('▶');
    expect(output).toContain('Agent 2');
    expect(output).toContain('Press Enter');
  });
});

  test('handles keyboard interaction', async () => {
    vi.spyOn(useMarketplaceModule, 'useMarketplace').mockReturnValue({
      agents: [
        { id: '1', name: 'Agent 1', description: 'Desc 1', author: 'Author 1', downloads: 100 },
        { id: '2', name: 'Agent 2', description: 'Desc 2', author: 'Author 2', downloads: 200 }
      ],
      loading: false,
      error: null,
      fetchAgents: vi.fn()
    });

    const onBack = vi.fn();
    const { lastFrame, stdin } = render(<Marketplace onBack={onBack} />);

    // Test DOWN arrow
    stdin.write('\x1B[B');
    let output = lastFrame();
    expect(output).toContain('Agent 2');

    // Test UP arrow
    stdin.write('\x1B[A');
    output = lastFrame();
    expect(output).toContain('Agent 1');

    // Test ENTER
    stdin.write('\r');
    expect(onBack).toHaveBeenCalledTimes(1);

    // Test Escape
    stdin.write('\x1B');
    expect(onBack).toHaveBeenCalledTimes(1);

    // Test 'q'
    stdin.write('q');
    expect(onBack).toHaveBeenCalledTimes(2);
  });
