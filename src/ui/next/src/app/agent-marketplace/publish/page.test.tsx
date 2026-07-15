import { describe, it, expect, vi, beforeEach } from "vitest";
import React from "react";
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import PublishAgentPage from './page';

/**
 * @jest-environment jsdom
 */

const mockFetch = vi.fn();
global.fetch = mockFetch;

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('Publish Agent Page', () => {
  beforeEach(() => {
    mockFetch.mockClear();
  });

  it('renders the publish agent form', async () => {
    render(<PublishAgentPage />);

    expect(screen.getByText('Publish New Agent')).toBeInTheDocument();
    expect(screen.getByLabelText('Agent Name')).toBeInTheDocument();
    expect(screen.getByLabelText('Description')).toBeInTheDocument();
    expect(screen.getByLabelText('Role')).toBeInTheDocument();
    expect(screen.getByLabelText('System Prompt')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Publish to Marketplace' })).toBeInTheDocument();
  });

  it('handles successful submission', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({ id: 'new-agent-id' }),
    });

    render(<PublishAgentPage />);

    fireEvent.change(screen.getByLabelText('Agent Name'), { target: { value: 'Test Agent' } });
    fireEvent.change(screen.getByLabelText('Description'), { target: { value: 'Test Desc' } });
    fireEvent.change(screen.getByLabelText('Role'), { target: { value: 'Tester' } });
    fireEvent.change(screen.getByLabelText('System Prompt'), { target: { value: 'You are a tester.' } });

    fireEvent.click(screen.getByRole('button', { name: 'Publish to Marketplace' }));

    await waitFor(() => {
      expect(mockFetch).toHaveBeenCalledWith('/api/v1/agents/marketplace', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: 'Test Agent',
          description: 'Test Desc',
          role: 'Tester',
          system_prompt: 'You are a tester.',
        }),
      });
    });
  });

  it('displays error on failed submission', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: false,
      json: async () => ({ error: 'Invalid agent data' }),
    });

    render(<PublishAgentPage />);

    fireEvent.change(screen.getByLabelText('Agent Name'), { target: { value: 'Bad Agent' } });
    fireEvent.change(screen.getByLabelText('Description'), { target: { value: 'Bad Desc' } });
    fireEvent.change(screen.getByLabelText('Role'), { target: { value: 'Bad Role' } });
    fireEvent.change(screen.getByLabelText('System Prompt'), { target: { value: 'Bad Prompt' } });

    const form = screen.getByRole('button', { name: 'Publish to Marketplace' }).closest('form');
    fireEvent.submit(form!);

    expect(await screen.findByText(/Failed to publish agent/i)).toBeInTheDocument();
  });

  it('displays error when data contains error', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({ error: 'Invalid agent data from backend' }),
    });

    render(<PublishAgentPage />);

    fireEvent.change(screen.getByLabelText('Agent Name'), { target: { value: 'Bad Agent' } });
    fireEvent.change(screen.getByLabelText('Description'), { target: { value: 'Bad Desc' } });
    fireEvent.change(screen.getByLabelText('Role'), { target: { value: 'Bad Role' } });
    fireEvent.change(screen.getByLabelText('System Prompt'), { target: { value: 'Bad Prompt' } });

    const form = screen.getByRole('button', { name: 'Publish to Marketplace' }).closest('form');
    fireEvent.submit(form!);

    expect(await screen.findByText('Invalid agent data from backend')).toBeInTheDocument();
  });
});
