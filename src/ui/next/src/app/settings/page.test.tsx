import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { vi } from 'vitest';
import SettingsPage from './page';
import { TooltipProvider } from '../../components/TooltipRegistry';

// Mock Next.js router and hooks
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
  usePathname: () => '/settings',
}));

describe('SettingsPage', () => {
  beforeEach(() => {
    // Mock fetch for the page initial load and subsequent actions
    global.fetch = vi.fn().mockImplementation((url) => {
      if (url === '/api/settings/delivery' || url === '/api/settings/voice' || url === '/api/settings/sms-preferences') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({}),
        });
      }
      return Promise.resolve({
        ok: true,
        json: () => Promise.resolve({}),
      });
    });
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('renders the Demo Data section and button', async () => {
    render(<TooltipProvider><SettingsPage /></TooltipProvider>);

    // The page initially shows a loading spinner, wait for it to disappear
    await waitFor(() => {
      expect(screen.queryByRole('status')).not.toBeInTheDocument();
    });

    const demoDataHeader = screen.getByText('Demo Data');
    expect(demoDataHeader).toBeInTheDocument();

    const reloadButton = screen.getByRole('button', { name: 'Reload Demo Data' });
    expect(reloadButton).toBeInTheDocument();
  });

  it('calls the reload demo data API when clicked and updates button text', async () => {
    render(<TooltipProvider><SettingsPage /></TooltipProvider>);

    await waitFor(() => {
      expect(screen.queryByRole('status')).not.toBeInTheDocument();
    });

    const reloadButton = screen.getByRole('button', { name: 'Reload Demo Data' });

    fireEvent.click(reloadButton);

    expect(reloadButton.innerText).toBe('Reloading...');
    expect(reloadButton).toBeDisabled();

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/dev/seed', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ scenario: 'launch-readiness' }),
      });
    });

    await waitFor(() => {
      expect(reloadButton.innerText).toBe('Success!');
    });
  });

  it('handles failed reload gracefully', async () => {
    (global.fetch as any).mockImplementation((url) => {
      if (url === '/api/dev/seed') {
        return Promise.resolve({ ok: false });
      }
      return Promise.resolve({ ok: true, json: () => Promise.resolve({}) });
    });

    render(<TooltipProvider><SettingsPage /></TooltipProvider>);

    await waitFor(() => {
      expect(screen.queryByRole('status')).not.toBeInTheDocument();
    });

    const reloadButton = screen.getByRole('button', { name: 'Reload Demo Data' });

    fireEvent.click(reloadButton);

    await waitFor(() => {
      expect(reloadButton.innerText).toBe('Failed');
    });
  });
});
