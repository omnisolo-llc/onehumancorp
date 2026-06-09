import { render, screen, fireEvent, act } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { ReviewFeedCard } from './ReviewFeedCard';

describe('ReviewFeedCard', () => {
  const review = {
    id: 'r1',
    rating: 5,
    content: 'Great service!',
    source: 'google',
    createdAtUnix: 1600000000,
  };

  const response = {
    id: 'resp1',
    draftedContent: 'Thank you for your feedback.',
    status: 'draft',
  };

  it('renders correctly', () => {
    render(<ReviewFeedCard review={review} response={response} onApprove={vi.fn()} onDismiss={vi.fn()} />);
    expect(screen.getByText('New 5-Star Review (google)')).toBeDefined();
    expect(screen.getByText('"Great service!"')).toBeDefined();
    expect(screen.getByText('Thank you for your feedback.')).toBeDefined();
  });

  it('calls onApprove when approve button is clicked', async () => {
    const onApprove = vi.fn().mockResolvedValue(undefined);
    render(<ReviewFeedCard review={review} response={response} onApprove={onApprove} onDismiss={vi.fn()} />);

    const btn = screen.getByText('Approve & Post');
    await act(async () => {
      fireEvent.click(btn);
    });
    expect(onApprove).toHaveBeenCalledWith('resp1', 'Thank you for your feedback.');
  });

  it('calls onDismiss when dismiss button is clicked', async () => {
    const onDismiss = vi.fn().mockResolvedValue(undefined);
    render(<ReviewFeedCard review={review} response={response} onApprove={vi.fn()} onDismiss={onDismiss} />);

    const btn = screen.getByText('Dismiss');
    await act(async () => {
      fireEvent.click(btn);
    });
    expect(onDismiss).toHaveBeenCalledWith('resp1');
  });

  it('can edit drafted content before approving', async () => {
    const onApprove = vi.fn().mockResolvedValue(undefined);
    render(<ReviewFeedCard review={review} response={response} onApprove={onApprove} onDismiss={vi.fn()} />);

    const editBtn = screen.getByText('Edit');
    await act(async () => {
      fireEvent.click(editBtn);
    });

    const textarea = screen.getByRole('textbox');
    await act(async () => {
      fireEvent.change(textarea, { target: { value: 'Edited response!' } });
    });

    const btn = screen.getByText('Approve & Post');
    await act(async () => {
      fireEvent.click(btn);
    });
    expect(onApprove).toHaveBeenCalledWith('resp1', 'Edited response!');
  });
});
