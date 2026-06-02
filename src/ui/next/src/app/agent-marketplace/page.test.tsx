import { render, screen, waitFor, act } from '@testing-library/react';
import AgentMarketplacePage from './page';
import { expect, test, vi, describe, beforeEach } from 'vitest';

global.fetch = vi.fn(() =>
  Promise.resolve({
    json: () => Promise.resolve([
      { id: "1", name: "The Promoter", description: "Marketing agent", author: "OHC", version: "1.0.0", endpoint: "test" },
      { id: "2", name: "The Manager", description: "Operations agent", author: "OHC", version: "1.0.0", endpoint: "test" },
      { id: "3", name: "The Accountant", description: "Finance agent", author: "OHC", version: "1.0.0", endpoint: "test" }
    ]),
    ok: true,
  })
) as any;

describe('Agent Marketplace Page', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  test('renders the marketplace header', async () => {
    await act(async () => {
      render(<AgentMarketplacePage />);
    });
    await waitFor(() => {
      expect(screen.getByText("Agent Marketplace")).toBeDefined();
    });
  });

  test('renders marketing agents', async () => {
    await act(async () => {
      render(<AgentMarketplacePage />);
    });
    await waitFor(() => {
      expect(screen.getByText("The Promoter")).toBeDefined();
    });
  });

  test('renders operations agents', async () => {
    await act(async () => {
      render(<AgentMarketplacePage />);
    });
    await waitFor(() => {
      expect(screen.getByText("The Manager")).toBeDefined();
    });
  });

  test('renders finance agents', async () => {
    await act(async () => {
      render(<AgentMarketplacePage />);
    });
    await waitFor(() => {
      expect(screen.getByText("The Accountant")).toBeDefined();
    });
  });
});
