import '@testing-library/jest-dom';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import ExpertTeamPage from './page';
import { vi } from 'vitest';

global.fetch = vi.fn();

describe('ExpertTeamPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders correctly', () => {
    render(<ExpertTeamPage />);
    expect(screen.getByText(/Collaborative Expert Team/i)).toBeInTheDocument();
  });

  it('handles execution and displays result', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ result: 'Test expert output' }),
    });

    render(<ExpertTeamPage />);

    const input = screen.getByPlaceholderText(/e.g. Write a comprehensive/i);
    fireEvent.change(input, { target: { value: 'Analyze market' } });

    const button = screen.getByRole('button', { name: /Execute Task/i });
    fireEvent.click(button);

    expect(global.fetch).toHaveBeenCalledWith('/api/expert-team', expect.any(Object));

    await waitFor(() => {
      expect(screen.getByText('Test expert output')).toBeInTheDocument();
    });
  });

  it('handles errors', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: false,
      json: async () => ({ error: 'Gate failed' }),
    });

    render(<ExpertTeamPage />);

    const input = screen.getByPlaceholderText(/e.g. Write a comprehensive/i);
    fireEvent.change(input, { target: { value: 'Analyze market' } });

    const button = screen.getByRole('button', { name: /Execute Task/i });
    fireEvent.click(button);

    await waitFor(() => {
      expect(screen.getByText('Gate failed')).toBeInTheDocument();
    });
  });
});
