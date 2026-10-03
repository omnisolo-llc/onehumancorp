import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import Page from './page';

import { vi } from 'vitest';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
    back: vi.fn(),
  }),
}));

vi.mock('../components/PoweredByOmniSolo', () => ({
  PoweredByOmniSolo: () => <div data-testid="powered-by-omnisolo">Powered By OmniSolo</div>,
}));

describe('Group Buy Widget Page', () => {
  beforeEach(() => {
    localStorage.clear();
    global.fetch = vi.fn().mockResolvedValue({ ok: true, json: async () => ({ current_plan: 'free' }) });
  });

  it('gives the selectable embed code its own accessible control', () => {
    render(<Page />);
    expect((screen.getByRole('textbox', { name: 'Embed Code' }) as HTMLTextAreaElement).value).toContain('<iframe');
    expect(screen.getByRole('button', { name: 'Copy Code' })).toBeEnabled();
  });

  it('marks unconfigured editor-preview actions unavailable with a visible reason', () => {
    render(<Page />);
    for(const name of ['Join Group Buy', 'Share with friends']) {
      const button=screen.getByRole('button',{name});
      expect(button).toBeDisabled();
      expect(button).toHaveAccessibleDescription('Joining and sharing are unavailable in this editor preview.');
    }
    expect(screen.getByText('Joining and sharing are unavailable in this editor preview.')).toBeVisible();
  });

  it('renders the builder with default values', async () => {
    render(<Page />);

    // Wait for client-side render
    await waitFor(() => {
      expect(screen.getByText('Group Buy Widget Builder')).toBeInTheDocument();
    });

    expect(screen.getByDisplayValue('Premium Coffee Subscription')).toBeInTheDocument();
    expect(screen.getByDisplayValue('24.99')).toBeInTheDocument();
    expect(screen.getByDisplayValue('15.00')).toBeInTheDocument();
  });

  it('shows paywall when removing branding without pro', async () => {
    render(<Page />);

    await waitFor(() => {
      expect(screen.getByText('Remove OmniSolo Branding')).toBeInTheDocument();
    });

    // Find the hidden checkbox associated with the label
    const brandingToggle = screen.getByRole('checkbox', { hidden: true });
    fireEvent.click(brandingToggle);

    expect(screen.getAllByText('Upgrade to Pro')[0]).toBeInTheDocument();
    expect(screen.getByText(/Removing OmniSolo branding/)).toBeInTheDocument();
  });

  it('allows removing branding with pro', async () => {
    global.fetch = vi.fn(async url => Response.json(url === '/api/v1/auth/session-identity' ? { userId: 'plan-owner', tenantId: 'plan-tenant', expiresAt: Date.now() + 60000 } : { current_plan: 'Pro' }));
    render(<Page />);

    await waitFor(() => {
      expect(screen.getByText('Remove OmniSolo Branding')).toBeInTheDocument();
      expect(global.fetch).toHaveBeenCalledWith('/api/v1/billing/my-plan', expect.objectContaining({ credentials: 'same-origin' }));
    });

    const brandingToggle = screen.getByRole('checkbox', { hidden: true });
    fireEvent.click(brandingToggle);

    expect(screen.queryByText('Upgrade to Pro')).not.toBeInTheDocument();
    expect(brandingToggle).toBeChecked();
  });

  it('updates embed code when inputs change', async () => {
    render(<Page />);

    await waitFor(() => {
      expect(screen.getByDisplayValue('Premium Coffee Subscription')).toBeInTheDocument();
    });

    const nameInput = screen.getByDisplayValue('Premium Coffee Subscription');
    fireEvent.change(nameInput, { target: { value: 'New Test Product' } });

    const textboxes = screen.getAllByRole('textbox');
    // the textarea is the last textbox
    const embedTextarea = textboxes[textboxes.length - 1] as HTMLTextAreaElement;
    expect(embedTextarea.value).toContain('New%20Test%20Product');
  });
});
