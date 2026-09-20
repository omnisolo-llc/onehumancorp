import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { expect, test, describe, vi, beforeEach } from 'vitest';
import InteractivePollGeneratorPage from './page';

const plan = vi.hoisted(() => ({ hasPro: false }));
vi.mock('../components/useProPlan', () => ({ useProPlan: () => plan }));

// Mock Next.js router
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

// Mock the PoweredByOmniSolo component to avoid complex rendering issues in tests
vi.mock('../components/PoweredByOmniSolo', () => ({
  PoweredByOmniSolo: () => <div data-testid="powered-by-omnisolo">Powered by OmniSolo</div>,
}));

describe('Interactive Poll Generator UI', () => {
  beforeEach(() => {
    // Clear localStorage between tests
    localStorage.clear();
    vi.clearAllMocks();
    plan.hasPro = false;
  });

  test('renders initial form and preview pane', () => {
    render(<InteractivePollGeneratorPage />);

    // Check main title
    expect(screen.getByText('Interactive Poll Generator')).toBeDefined();

    // Check default inputs
    const questionInputs = screen.queryAllByPlaceholderText('E.g., What should we build next?');
    expect(questionInputs.length).toBeGreaterThan(0);
    expect((questionInputs[0] as HTMLInputElement).value).toBe('What flavor should we make next?');

    // Check default preview
    expect(screen.queryAllByText('What flavor should we make next?').length).toBeGreaterThan(0);
    expect(screen.queryAllByText('Chocolate').length).toBeGreaterThan(0);
    expect(screen.queryAllByText('Vanilla').length).toBeGreaterThan(0);
    expect(screen.queryAllByText('Strawberry').length).toBeGreaterThan(0);

    // Check Powered By OmniSolo is visible by default
    expect(screen.getByTestId('powered-by-omnisolo')).toBeDefined();
  });

  test('allows changing the question and options', () => {
    render(<InteractivePollGeneratorPage />);

    const questionInputs = screen.queryAllByPlaceholderText('E.g., What should we build next?');
    const questionInput = questionInputs[0];
    fireEvent.change(questionInput, { target: { value: 'Favorite color?' } });

    // Both input and preview should update
    expect((questionInput as HTMLInputElement).value).toBe('Favorite color?');
    expect(screen.getAllByText('Favorite color?').length).toBeGreaterThan(0);

    const option1Inputs = screen.queryAllByPlaceholderText('Option 1');
    const option1Input = option1Inputs.length > 0 ? option1Inputs[0] : null;

    if (option1Input) {
        fireEvent.change(option1Input, { target: { value: 'Red' } });
        expect(screen.getAllByText('Red').length).toBeGreaterThan(0);
    }
  });

  test('toggles email requirement in preview', () => {
    render(<InteractivePollGeneratorPage />);

    const emailCheckbox = screen.getAllByRole('checkbox')[0]; // First checkbox is email requirement

    // Initially, no email input in preview
    expect(screen.queryByPlaceholderText('Enter your email to vote')).toBeNull();

    // Toggle on
    fireEvent.click(emailCheckbox);
    expect(screen.getByPlaceholderText('Enter your email to vote')).toBeDefined();
  });

  test('shows embed modal when generate button is clicked', () => {
    render(<InteractivePollGeneratorPage />);

    const generateBtn = screen.getAllByText('Generate Embed Code')[0];
    fireEvent.click(generateBtn);

    // Modal appears
    expect(screen.getByText('Your Embed Code')).toBeDefined();
    expect(screen.getByText(/<iframe/)).toBeDefined();

    // Close modal
    const closeBtn = screen.getAllByText('Close')[0];
    fireEvent.click(closeBtn);
    expect(screen.queryByText('Your Embed Code')).toBeNull();
  });

  test('shows paywall when removing branding without pro', () => {
    // Ensure pro is false
    localStorage.setItem('has_pro', 'false');

    render(<InteractivePollGeneratorPage />);

    const removeBrandingCheckbox = screen.getAllByRole('checkbox')[1]; // Second checkbox is remove branding
    fireEvent.click(removeBrandingCheckbox);

    // Paywall modal appears
    expect(screen.getAllByText('Upgrade to Pro').length).toBeGreaterThan(0);
    expect(screen.getAllByText('View Pro Plans').length).toBeGreaterThan(0);
  });

  test('allows removing branding with pro', () => {
    plan.hasPro = true;
    render(<InteractivePollGeneratorPage />);
    const branding = screen.getAllByRole('checkbox')[1];
    fireEvent.click(branding);
    expect(branding).toBeChecked();
    expect(screen.queryByText('View Pro Plans')).not.toBeInTheDocument();
    fireEvent.click(screen.getAllByText('Generate Embed Code')[0]);
    expect(screen.getByText(/<iframe/).textContent).toContain('hideBranding=true');
    expect(screen.getByText(/<iframe/).textContent).not.toContain('Powered by OmniSolo');
  });
});
