import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { ContextCard } from './ContextCard';
import '@testing-library/jest-dom';

// Mock fetch
global.fetch = vi.fn();

describe('ContextCard', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders loading state initially', () => {
    (global.fetch as any).mockImplementationOnce(() =>
      new Promise(() => {}) // Never resolves to keep it in loading state
    );

    render(<ContextCard tenantId="t1" customerId="c1" />);
    expect(screen.getByText('Loading context...')).toBeInTheDocument();
  });

  it('renders correctly with data', async () => {
    const mockData = {
      total_interactions: 5,
      last_interaction: '2023-01-01T10:00:00Z',
      segments: ['VIP'],
      preferences: ['Email'],
      summary: 'A loyal customer.'
    };

    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => mockData
    });

    render(<ContextCard tenantId="t1" customerId="c1" />);

    await waitFor(() => {
      expect(screen.getByText('VIP')).toBeInTheDocument();
    });

    expect(screen.getByText('Email')).toBeInTheDocument();
    expect(screen.getByText(/5 past orders/)).toBeInTheDocument();
  });

  it('renders fallback when no segments or preferences', async () => {
    const mockData = {
      total_interactions: 0,
      last_interaction: null,
      segments: [],
      preferences: [],
      summary: 'New customer.'
    };

    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => mockData
    });

    render(<ContextCard tenantId="t1" customerId="c1" />);

    await waitFor(() => {
      expect(screen.getByText('New customer.')).toBeInTheDocument();
    });

    expect(screen.getByText(/0 past orders/)).toBeInTheDocument();
    expect(screen.getByText(/Last order: N\/A/)).toBeInTheDocument();
  });
});
