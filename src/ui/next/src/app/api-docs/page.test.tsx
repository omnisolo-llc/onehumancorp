import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import ApiDocsPage from './page';

vi.mock('swagger-ui-react', () => {
  return {
    default: ({ spec }: { spec: any }) => <div data-testid="swagger-ui">{spec ? spec.info.title : 'Loading'}</div>
  };
});

describe('ApiDocsPage', () => {
  beforeEach(() => {
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('renders the advanced warning banner', async () => {
    (global.fetch as any).mockResolvedValue({
      ok: true,
      json: async () => ({ info: { title: 'Test API' } })
    });

    render(<ApiDocsPage />);
    expect(screen.getByText(/Advanced:/)).toBeInTheDocument();
    await waitFor(() => {
        expect(screen.getByTestId('swagger-ui')).toBeInTheDocument();
    });
  });

  it('fetches and renders the swagger spec', async () => {
    (global.fetch as any).mockResolvedValue({
      ok: true,
      json: async () => ({ info: { title: 'OHC Advanced API Reference' } })
    });

    render(<ApiDocsPage />);

    await waitFor(() => {
      expect(screen.getByTestId('swagger-ui')).toBeInTheDocument();
      expect(screen.getByText('OHC Advanced API Reference')).toBeInTheDocument();
    });
  });

  it('displays an error message if fetch fails', async () => {
    (global.fetch as any).mockResolvedValue({
      ok: false
    });

    render(<ApiDocsPage />);

    await waitFor(() => {
      expect(screen.getByText('Failed to load API spec')).toBeInTheDocument();
    });
  });
});
