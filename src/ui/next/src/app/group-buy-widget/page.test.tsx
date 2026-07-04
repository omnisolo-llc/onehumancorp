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

vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc">Powered By OHC</div>,
}));

describe('Group Buy Widget Page', () => {
  beforeEach(() => {
    localStorage.clear();
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
    localStorage.setItem('has_pro', 'false');
    render(<Page />);

    await waitFor(() => {
      expect(screen.getByText('Remove OHC Branding')).toBeInTheDocument();
    });

    // Find the hidden checkbox associated with the label
    const brandingToggle = screen.getByRole('checkbox', { hidden: true });
    fireEvent.click(brandingToggle);

    expect(screen.getAllByText('Upgrade to Pro')[0]).toBeInTheDocument();
    expect(screen.getByText(/Removing OHC branding/)).toBeInTheDocument();
  });

  it('allows removing branding with pro', async () => {
    localStorage.setItem('has_pro', 'true');
    render(<Page />);

    await waitFor(() => {
      expect(screen.getByText('Remove OHC Branding')).toBeInTheDocument();
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
