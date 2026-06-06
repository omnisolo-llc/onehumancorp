import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import { ConversationalIntake } from '../ConversationalIntake';
import { useOnboardingStore } from '../../store';
import { TooltipProvider } from '../../../../components/TooltipRegistry';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import userEvent from '@testing-library/user-event';

describe('ConversationalIntake', () => {
  const renderComponent = () => render(
    <TooltipProvider>
      <ConversationalIntake onSaveDraft={vi.fn()} />
    </TooltipProvider>
  );

  beforeEach(() => {
    useOnboardingStore.setState({
      chatStep: 1,
      businessName: '',
      whatYouSell: '',
      location: '',
      isLoading: false,
    });
  });

  it('renders chat step 1: business name', () => {
    renderComponent();
    expect(screen.getByText(/What's the name of your business/i)).toBeInTheDocument();
    expect(screen.getByPlaceholderText(/Maya's Custom Cakes/i)).toBeInTheDocument();
  });

  it('updates business name on input', async () => {
    const user = userEvent.setup();
    renderComponent();
    const input = screen.getByPlaceholderText(/Maya's Custom Cakes/i);
    await user.type(input, 'My Test Biz');
    expect(useOnboardingStore.getState().businessName).toBe('My Test Biz');
  });

  it('shows validation error for short business name', async () => {
    const user = userEvent.setup();
    renderComponent();
    const input = screen.getByPlaceholderText(/Maya's Custom Cakes/i);
    await user.type(input, 'Ab{Enter}');
    expect(screen.getByText(/Business Name must be at least 3 characters/i)).toBeInTheDocument();
  });
});
