/** @jsxImportSource react */
import { render, screen, waitFor, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { NeighborhoodPulseCard } from './NeighborhoodPulseCard';
import React from 'react';

describe('NeighborhoodPulseCard', () => {
  beforeEach(() => {
    global.fetch = vi.fn();
    vi.spyOn(window, 'alert').mockImplementation(() => {});
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders nothing when loading', () => {
    (global.fetch as any).mockImplementation(() => new Promise(() => {})); // Never resolves

    const { container } = render(<NeighborhoodPulseCard tenant="test-tenant" />);
    expect(container.firstChild).toBeNull();
  });

  it('renders nothing when there are no neighbors', async () => {
    (global.fetch as any).mockResolvedValue({
      json: () => Promise.resolve({ neighbors: [] })
    });

    const { container } = render(<NeighborhoodPulseCard tenant="test-tenant" />);

    await waitFor(() => {
      expect(container.firstChild).toBeNull();
    });
  });

  it('handles fetch exception and renders nothing', async () => {
    (global.fetch as any).mockRejectedValue(new Error('Network error'));

    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    const { container } = render(<NeighborhoodPulseCard tenant="test-tenant" />);

    await waitFor(() => {
      expect(container.firstChild).toBeNull();
      expect(consoleErrorSpy).toHaveBeenCalled();
    });
  });

  it('renders correctly with neighbors and asserts visual styles', async () => {
    (global.fetch as any).mockResolvedValue({
      json: () => Promise.resolve({ neighbors: ['neighbor_one', 'neighbor_two'] })
    });

    const { container } = render(<NeighborhoodPulseCard tenant="test-tenant" />);

    await waitFor(() => {
      expect(screen.getByText('Neighborhood Pulse')).toBeInTheDocument();
      expect(screen.getByText(/There are 2 OHC businesses/)).toBeInTheDocument();
      expect(screen.getByText('Neighbor One')).toBeInTheDocument();
      expect(screen.getByText('Neighbor Two')).toBeInTheDocument();
    });

    const wrapper = container.firstChild as HTMLElement;
    expect(wrapper).toHaveClass('backdrop-blur-[30px]');
    expect(wrapper).toHaveClass('saturate-[210%]');
    expect(wrapper).toHaveClass('bg-white/65');
  });

  it('handles successful invitation', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url.includes('action=getNearby')) {
        return Promise.resolve({
          json: () => Promise.resolve({ neighbors: ['neighbor_one'] })
        });
      }
      if (url.includes('/api/mesh/v2/collective')) {
        return Promise.resolve({
          json: () => Promise.resolve({ success: true })
        });
      }
      return Promise.reject(new Error('not found'));
    });

    render(<NeighborhoodPulseCard tenant="test-tenant" />);

    await waitFor(() => {
      expect(screen.getByText('Neighbor One')).toBeInTheDocument();
    });

    const inviteBtn = screen.getByText('Invite Partner');
    fireEvent.click(inviteBtn);

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/mesh/v2/collective', expect.objectContaining({
        method: 'POST',
        body: JSON.stringify({ action: 'invite', target_tenant_id: 'neighbor_one' })
      }));
      expect(window.alert).toHaveBeenCalledWith('Invitation sent successfully!');
    });
  });

  it('handles failed invitation (success: false)', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url.includes('action=getNearby')) {
        return Promise.resolve({
          json: () => Promise.resolve({ neighbors: ['neighbor_one'] })
        });
      }
      if (url.includes('/api/mesh/v2/collective')) {
        return Promise.resolve({
          json: () => Promise.resolve({ success: false })
        });
      }
      return Promise.reject(new Error('not found'));
    });

    render(<NeighborhoodPulseCard tenant="test-tenant" />);

    await waitFor(() => {
      expect(screen.getByText('Neighbor One')).toBeInTheDocument();
    });

    const inviteBtn = screen.getByText('Invite Partner');
    fireEvent.click(inviteBtn);

    await waitFor(() => {
      expect(window.alert).toHaveBeenCalledWith('Failed to send invitation');
    });
  });

  it('handles invitation network exception', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url.includes('action=getNearby')) {
        return Promise.resolve({
          json: () => Promise.resolve({ neighbors: ['neighbor_one'] })
        });
      }
      if (url.includes('/api/mesh/v2/collective')) {
        return Promise.reject(new Error('Network down'));
      }
      return Promise.reject(new Error('not found'));
    });

    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    render(<NeighborhoodPulseCard tenant="test-tenant" />);

    await waitFor(() => {
      expect(screen.getByText('Neighbor One')).toBeInTheDocument();
    });

    const inviteBtn = screen.getByText('Invite Partner');
    fireEvent.click(inviteBtn);

    await waitFor(() => {
      expect(consoleErrorSpy).toHaveBeenCalled();
      expect(window.alert).toHaveBeenCalledWith('Error occurred while inviting');
    });
  });

  it('handles missing neighbors data', async () => {
    (global.fetch as any).mockResolvedValue({
      json: () => Promise.resolve({ not_neighbors: [] })
    });

    const { container } = render(<NeighborhoodPulseCard tenant="test-tenant" />);

    await waitFor(() => {
      expect(container.firstChild).toBeNull();
    });
  });
});
