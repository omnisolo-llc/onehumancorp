import { render, screen, fireEvent, act } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import CloudBridgeInviteModal from './CloudBridgeInviteModal';

describe('CloudBridgeInviteModal', () => {
  it('renders correctly', () => {
    const onClose = vi.fn();
    render(<CloudBridgeInviteModal onClose={onClose} />);

    expect(screen.getByRole('heading', { name: 'Cloud Bridge Invite' })).toBeDefined();
    expect(screen.getByText('Share this link to provision a temporary multi-tenant context')).toBeDefined();
    expect(screen.getByRole('button', { name: 'Copy Link' })).toBeDefined();
    expect(screen.getByRole('button', { name: 'Close Cloud Bridge Invite' })).toBeDefined();
  });

  it('calls onClose when close button is clicked', () => {
    const onClose = vi.fn();
    render(<CloudBridgeInviteModal onClose={onClose} />);

    fireEvent.click(screen.getByRole('button', { name: 'Close Cloud Bridge Invite' }));
    expect(onClose).toHaveBeenCalled();
  });

  it('handles copy link button click', async () => {
    const onClose = vi.fn();
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockImplementation(() => Promise.resolve()),
      },
    });

    render(<CloudBridgeInviteModal onClose={onClose} />);

    vi.useFakeTimers();

    const copyButton = screen.getByRole('button', { name: 'Copy Link' });

    await act(async () => {
      fireEvent.click(copyButton);
    });

    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    expect(screen.getByRole('button', { name: 'Copied!' })).toBeDefined();

    await act(async () => {
      vi.advanceTimersByTime(2000);
    });

    expect(screen.getByRole('button', { name: 'Copy Link' })).toBeDefined();

    vi.useRealTimers();
  });
});
