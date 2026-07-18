import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import MultilingualOrderInterceptor from './page';

describe('MultilingualOrderInterceptor', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
    vi.unstubAllGlobals();
  });

  it('does not fabricate a voice transcript when transcription is unavailable', () => {
    render(<MultilingualOrderInterceptor />);

    expect(screen.getByRole('button', { name: 'Voice transcription unavailable' })).toBeDisabled();
    expect(screen.getByText('Voice transcription is unavailable. Type the order instead.')).toBeInTheDocument();
    expect(screen.queryByDisplayValue('Quiero 3 tacos de pollo')).not.toBeInTheDocument();
  });

  it('processes user-entered text through the real order interceptor endpoint', async () => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({ language: 'Spanish', intent: 'Order', items: [] }),
    }));
    render(<MultilingualOrderInterceptor />);

    fireEvent.change(screen.getByPlaceholderText('Type order here...'), {
      target: { value: 'Dos cafés' },
    });
    fireEvent.click(screen.getByRole('button', { name: 'Process Text Order' }));

    await waitFor(() => expect(global.fetch).toHaveBeenCalledWith(
      '/api/v1/agents/order-interceptor',
      expect.objectContaining({ body: JSON.stringify({ raw_input: 'Dos cafés' }) }),
    ));
  });
});
