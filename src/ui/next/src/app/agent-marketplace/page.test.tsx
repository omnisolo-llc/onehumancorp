import { describe, it, expect, vi, beforeEach } from "vitest";
import React from "react";

/**
 * @jest-environment jsdom
 */

import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
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
          name: 'Data Analyst',
          description: 'Analyzes CSV files',
          author: 'AutoGPT',
          version: '1.0.0',
          endpoint: 'https://marketplace.example.com/agents/agent-1',
        },
      ],
    });
  });

  it('renders the marketplace header', async () => {
    await act(async () => { render(<AgentMarketplacePage />); });
    expect(screen.getByText('Agent Marketplace')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Search for agents...')).toBeInTheDocument();
  });

  it('fetches and displays agents on load', async () => {
    await act(async () => { render(<AgentMarketplacePage />); });

    await waitFor(() => {
      expect(screen.getByText('Data Analyst')).toBeInTheDocument();
    });
    expect(screen.getByText('Analyzes CSV files')).toBeInTheDocument();
    expect(screen.getByText('AutoGPT')).toBeInTheDocument();

  });

  it('displays a no results message when no agents are returned', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => [],
    });

    await act(async () => { render(<AgentMarketplacePage />); });

    await waitFor(() => {
      expect(screen.getByText(/No agents found matching/)).toBeInTheDocument();
    });
  });

  it('updates the search query when typing', async () => {
    await act(async () => { render(<AgentMarketplacePage />); });

    const searchInput = screen.getByPlaceholderText('Search for agents...');
    fireEvent.change(searchInput, { target: { value: 'SEO' } });

    expect(searchInput).toHaveValue('SEO');

    await waitFor(() => {
      expect(mockFetch).toHaveBeenCalledWith('/api/marketplace?q=SEO');
    });
  });

  it('displays an error message when fetch fails', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: false,
    });

    await act(async () => { render(<AgentMarketplacePage />); });

    await waitFor(() => {
      expect(screen.getByText('Failed to fetch agents')).toBeInTheDocument();
    });
  });
});
