import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import { ReviewStage } from '../ReviewStage';
import { useOnboardingStore } from '../../store';
import { TooltipProvider } from '../../../../components/TooltipRegistry';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import userEvent from '@testing-library/user-event';

describe('ReviewStage', () => {
  const renderComponent = () => render(
    <TooltipProvider>
      <ReviewStage onSaveDraft={vi.fn()} />
    </TooltipProvider>
  );

  beforeEach(() => {
    useOnboardingStore.setState({
      businessName: 'Original Name',
      businessType: 'Original Type',
      categories: ['cat1'],
      firstProductName: 'Product',
      firstProductPrice: '10',
      saveMessage: '',
    });
  });

  it('renders business name and type', () => {
    renderComponent();
    expect(screen.getByDisplayValue('Original Name')).toBeInTheDocument();
    expect(screen.getByDisplayValue('Original Type')).toBeInTheDocument();
  });

  it('validates business name length', async () => {
    const user = userEvent.setup();
    renderComponent();
    const input = screen.getByDisplayValue('Original Name');
    await user.clear(input);
    await user.type(input, 'Ab');
    expect(screen.getByText(/Must be at least 3 characters/i)).toBeInTheDocument();
  });
});
