import { describe, it, expect, vi, beforeEach } from "vitest";
import React from "react";

/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import AgentMarketplacePage from './page';

// Mock the global fetch API
const mockFetch = vi.fn();
global.fetch = mockFetch;

describe('Agent Marketplace Page', () => {
  beforeEach(() => {
    mockFetch.mockClear();
    mockFetch.mockResolvedValue({
      ok: true,
      json: async () => [
        {
          id: 'agent-1',
          name: 'Senior Rust Developer',
          description: 'An expert in Rust capable of building concurrent and safe systems.',
          author: 'AutoGPT',
          version: '1.0.0',
          endpoint: 'https://marketplace.example.com/agents/agent-1',
        },
      ],
    });
  });

  it('renders the marketplace header', async () => {
    render(<AgentMarketplacePage />);

    await waitFor(() => {
      expect(screen.getByText('Agent Marketplace')).toBeInTheDocument();
    });
    expect(screen.getByPlaceholderText('Search for agents...')).toBeInTheDocument();
  });

  it('fetches and displays agents on load', async () => {
    render(<AgentMarketplacePage />);

    await waitFor(() => {
      expect(screen.getByText('Senior Rust Developer')).toBeInTheDocument();
    });
    expect(screen.getByText('An expert in Rust capable of building concurrent and safe systems.')).toBeInTheDocument();
    expect(screen.getByText(/By AutoGPT/)).toBeInTheDocument();

  });

  it('displays a no results message when no agents are returned', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => [],
    });

    render(<AgentMarketplacePage />);

    await waitFor(() => {
      expect(screen.getByText(/No agents found/)).toBeInTheDocument();
    });
  });

  it('updates the search query when typing', async () => {
    render(<AgentMarketplacePage />);

    const searchInput = screen.getByPlaceholderText('Search for agents...');
    fireEvent.change(searchInput, { target: { value: 'SEO' } });

    expect(searchInput).toHaveValue('SEO');

    await waitFor(() => {
      expect(mockFetch).toHaveBeenCalledWith('/api/agents/marketplace?q=SEO');
    });
  });

  it('displays an error message when fetch fails', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: false,
    });

    render(<AgentMarketplacePage />);

    await waitFor(() => {
      expect(screen.getByText(/Failed to fetch agents/)).toBeInTheDocument();
    });
  });
});
