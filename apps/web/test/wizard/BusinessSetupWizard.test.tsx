import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import BusinessSetupWizard from '../../src/components/wizard/BusinessSetupWizard';
import '@testing-library/jest-dom';

// ---------------------------------------------------------------------------
// Helper: advance wizard to step N by clicking "Next" N-1 times
// ---------------------------------------------------------------------------
const advanceToStep = (n: number) => {
  for (let i = 1; i < n; i++) {
    fireEvent.click(screen.getByText('Next'));
  }
};

describe('BusinessSetupWizard', () => {
  beforeEach(() => {
    localStorage.clear();
    global.fetch = jest.fn().mockResolvedValue({ ok: true, json: () => Promise.resolve({}) });
    global.alert = jest.fn();
  });

  afterEach(() => {
    jest.restoreAllMocks();
  });

  // ── Step 1 – Welcome ────────────────────────────────────────────────────

  it('renders welcome screen on first load', () => {
    render(<BusinessSetupWizard />);
    expect(screen.getByText('Your AI team, ready in minutes')).toBeInTheDocument();
  });

  it('navigates forward from step 1 to step 2', () => {
    render(<BusinessSetupWizard />);
    fireEvent.click(screen.getByText('Next'));
    expect(screen.getByText('Business Profile')).toBeInTheDocument();
  });

  // ── Step 2 – Business Profile ───────────────────────────────────────────

  it('renders Business Profile step with all inputs', () => {
    render(<BusinessSetupWizard />);
    advanceToStep(2);
    expect(screen.getByPlaceholderText('Company Name')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Language')).toBeInTheDocument();
  });

  it('updates profile name on input change', () => {
    render(<BusinessSetupWizard />);
    advanceToStep(2);
    fireEvent.change(screen.getByPlaceholderText('Company Name'), { target: { value: 'Acme Corp' } });
    expect(screen.getByDisplayValue('Acme Corp')).toBeInTheDocument();
  });

  it('updates profile industry via select', () => {
    render(<BusinessSetupWizard />);
    advanceToStep(2);
    const selects = screen.getAllByRole('combobox');
    fireEvent.change(selects[0], { target: { value: 'tech' } });
    expect(selects[0]).toHaveValue('tech');
  });

  it('updates profile size via select', () => {
    render(<BusinessSetupWizard />);
    advanceToStep(2);
    const selects = screen.getAllByRole('combobox');
    fireEvent.change(selects[1], { target: { value: 'M' } });
    expect(selects[1]).toHaveValue('M');
  });

  it('updates language on input change', () => {
    render(<BusinessSetupWizard />);
    advanceToStep(2);
    fireEvent.change(screen.getByPlaceholderText('Language'), { target: { value: 'English' } });
    expect(screen.getByDisplayValue('English')).toBeInTheDocument();
  });

  it('navigates back from step 2 to step 1', () => {
    render(<BusinessSetupWizard />);
    advanceToStep(2);
    fireEvent.click(screen.getByText('Back'));
    expect(screen.getByText('Your AI team, ready in minutes')).toBeInTheDocument();
  });

  // ── Step 3 – Goal Selection ─────────────────────────────────────────────

  it('renders Goal Selection step', () => {
    render(<BusinessSetupWizard />);
    advanceToStep(3);
    expect(screen.getByText('Goal Selection')).toBeInTheDocument();
  });

  it('toggles a goal on and off', () => {
    render(<BusinessSetupWizard />);
    advanceToStep(3);
    const cb = screen.getByLabelText(/Automate customer support/i);
    fireEvent.click(cb);
    expect(cb).toBeChecked();
    fireEvent.click(cb);
    expect(cb).not.toBeChecked();
  });

  it('selects multiple goals', () => {
    render(<BusinessSetupWizard />);
    advanceToStep(3);
    fireEvent.click(screen.getByLabelText(/Build software faster/i));
    fireEvent.click(screen.getByLabelText(/Generate marketing content/i));
    fireEvent.click(screen.getByLabelText(/Analyze data/i));
    expect(screen.getByLabelText(/Build software faster/i)).toBeChecked();
    expect(screen.getByLabelText(/Generate marketing content/i)).toBeChecked();
    expect(screen.getByLabelText(/Analyze data/i)).toBeChecked();
  });

  it('toggles the Custom goal', () => {
    render(<BusinessSetupWizard />);
    advanceToStep(3);
    // Use getAllByRole and find the one whose accessible name is exactly "Custom"
    const checkboxes = screen.getAllByRole('checkbox');
    const customCheckbox = checkboxes.find((cb) => cb.getAttribute('type') === 'checkbox' &&
      cb.closest('label')?.textContent?.trim() === 'Custom'
    );
    expect(customCheckbox).toBeDefined();
    fireEvent.click(customCheckbox!);
    expect(customCheckbox).toBeChecked();
    fireEvent.click(customCheckbox!);
    expect(customCheckbox).not.toBeChecked();
  });

  it('navigates back from step 4 to step 3', () => {
    render(<BusinessSetupWizard />);
    advanceToStep(4);
    fireEvent.click(screen.getByText('Back'));
    expect(screen.getByText('Goal Selection')).toBeInTheDocument();
  });

  it('navigates back from step 5 to step 4', () => {
    render(<BusinessSetupWizard />);
    advanceToStep(5);
    fireEvent.click(screen.getByText('Back'));
    expect(screen.getByText('Deployment Preference')).toBeInTheDocument();
  });

  it('navigates back from step 6 to step 5', () => {
    render(<BusinessSetupWizard />);
    advanceToStep(6);
    fireEvent.click(screen.getByText('Back'));
    expect(screen.getByText('Administrator Account')).toBeInTheDocument();
  });

  // ── Step 4 – Deployment Preference ─────────────────────────────────────

  it('renders Deployment Preference step', () => {
    render(<BusinessSetupWizard />);
    advanceToStep(4);
    expect(screen.getByText('Deployment Preference')).toBeInTheDocument();
  });

  it('updates deployment via select', () => {
    render(<BusinessSetupWizard />);
    advanceToStep(4);
    const select = screen.getByRole('combobox');
    fireEvent.change(select, { target: { value: 'desktop' } });
    expect(select).toHaveValue('desktop');
  });

  // ── Step 5 – Administrator Account ────────────────────────────────────

  it('renders Administrator Account step', () => {
    render(<BusinessSetupWizard />);
    advanceToStep(5);
    expect(screen.getByText('Administrator Account')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Name')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Email')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Password')).toBeInTheDocument();
  });

  it('updates admin fields', () => {
    render(<BusinessSetupWizard />);
    advanceToStep(5);
    fireEvent.change(screen.getByPlaceholderText('Name'), { target: { value: 'Alice' } });
    fireEvent.change(screen.getByPlaceholderText('Email'), { target: { value: 'alice@example.com' } });
    fireEvent.change(screen.getByPlaceholderText('Password'), { target: { value: 'secret123' } });
    expect(screen.getByDisplayValue('Alice')).toBeInTheDocument();
    expect(screen.getByDisplayValue('alice@example.com')).toBeInTheDocument();
    expect(screen.getByDisplayValue('secret123')).toBeInTheDocument();
  });

  // ── Step 6 – Review & Launch ────────────────────────────────────────────

  it('renders Review & Launch step', () => {
    render(<BusinessSetupWizard />);
    advanceToStep(6);
    expect(screen.getByText('Review & Launch')).toBeInTheDocument();
  });

  it('calls /api/provision on launch', async () => {
    render(<BusinessSetupWizard />);
    advanceToStep(6);
    fireEvent.click(screen.getByRole('button', { name: /Launch My AI Team/i }));
    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/provision', expect.any(Object));
    });
  });

  it('shows "Launching…" while submitting', () => {
    global.fetch = jest.fn(() => new Promise(() => {}));
    render(<BusinessSetupWizard />);
    advanceToStep(6);
    fireEvent.click(screen.getByRole('button', { name: /Launch My AI Team/i }));
    expect(screen.getByText('Launching...')).toBeInTheDocument();
  });

  it('shows success alert when API returns ok', async () => {
    render(<BusinessSetupWizard />);
    advanceToStep(6);
    fireEvent.click(screen.getByRole('button', { name: /Launch My AI Team/i }));
    await waitFor(() => expect(global.alert).toHaveBeenCalledWith('Launched!'));
  });

  it('shows failure alert when API returns non-ok', async () => {
    global.fetch = jest.fn().mockResolvedValue({ ok: false });
    render(<BusinessSetupWizard />);
    advanceToStep(6);
    fireEvent.click(screen.getByRole('button', { name: /Launch My AI Team/i }));
    await waitFor(() => expect(global.alert).toHaveBeenCalledWith('Failed to launch'));
  });

  it('shows error alert when API throws', async () => {
    global.fetch = jest.fn().mockRejectedValue(new Error('Network error'));
    render(<BusinessSetupWizard />);
    advanceToStep(6);
    fireEvent.click(screen.getByRole('button', { name: /Launch My AI Team/i }));
    await waitFor(() => expect(global.alert).toHaveBeenCalledWith('Error launching'));
  });

  // ── Expert Mode ─────────────────────────────────────────────────────────

  it('toggles expert mode on and off', () => {
    render(<BusinessSetupWizard />);
    const toggle = screen.getByLabelText('Expert Mode');
    fireEvent.click(toggle);
    expect(screen.getByText('Raw Config Fields Revealed')).toBeInTheDocument();
    fireEvent.click(toggle);
    expect(screen.queryByText('Raw Config Fields Revealed')).not.toBeInTheDocument();
  });

  it('persists expertMode=true to localStorage', () => {
    render(<BusinessSetupWizard />);
    fireEvent.click(screen.getByLabelText('Expert Mode'));
    expect(localStorage.getItem('expertMode')).toBe('true');
  });

  it('persists expertMode=false to localStorage when unchecked', () => {
    localStorage.setItem('expertMode', 'true');
    render(<BusinessSetupWizard />);
    const toggle = screen.getByLabelText('Expert Mode');
    fireEvent.click(toggle); // uncheck
    expect(localStorage.getItem('expertMode')).toBe('false');
  });

  it('restores expertMode from localStorage on mount', async () => {
    localStorage.setItem('expertMode', 'true');
    render(<BusinessSetupWizard />);
    await waitFor(() => expect(screen.getByText('Raw Config Fields Revealed')).toBeInTheDocument());
  });

  // ── Full CUJ ────────────────────────────────────────────────────────────

  it('completes the full wizard flow end-to-end', async () => {
    render(<BusinessSetupWizard />);

    // Step 1
    fireEvent.click(screen.getByText('Next'));

    // Step 2 – fill profile
    fireEvent.change(screen.getByPlaceholderText('Company Name'), { target: { value: 'TestCo' } });
    const selects = screen.getAllByRole('combobox');
    fireEvent.change(selects[0], { target: { value: 'finance' } });
    fireEvent.change(selects[1], { target: { value: 'L' } });
    fireEvent.click(screen.getByText('Next'));

    // Step 3 – goals
    fireEvent.click(screen.getByLabelText(/Automate customer support/i));
    fireEvent.click(screen.getByLabelText(/Build software faster/i));
    fireEvent.click(screen.getByText('Next'));

    // Step 4 – deployment
    fireEvent.click(screen.getByText('Next'));

    // Step 5 – admin
    fireEvent.change(screen.getByPlaceholderText('Name'), { target: { value: 'Admin' } });
    fireEvent.change(screen.getByPlaceholderText('Email'), { target: { value: 'admin@testco.com' } });
    fireEvent.change(screen.getByPlaceholderText('Password'), { target: { value: 'pass' } });
    fireEvent.click(screen.getByText('Next'));

    // Step 6 – launch
    fireEvent.click(screen.getByRole('button', { name: /Launch My AI Team/i }));

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith(
        '/api/provision',
        expect.objectContaining({
          method: 'POST',
          body: expect.stringContaining('TestCo'),
        }),
      );
    });
  });
});
