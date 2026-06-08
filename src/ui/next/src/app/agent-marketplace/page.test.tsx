import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import AgentMarketplacePage from './page';
import { vi } from 'vitest';

// Mock fetch
global.fetch = vi.fn() as any;

describe('Agent Marketplace Page', () => {
  beforeEach(() => {
    (global.fetch as any).mockClear();
  });

  it('renders correctly and fetches agents', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => [
        {
          id: '1',
          name: 'Data Analyst',
          description: 'Analyzes data',
          author: 'OHC',
          version: '1.0',
        },
      ],
    });

    render(<AgentMarketplacePage />);

    expect(screen.getByText('Agent Marketplace')).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText('Data Analyst')).toBeInTheDocument();
    });
  });
});
