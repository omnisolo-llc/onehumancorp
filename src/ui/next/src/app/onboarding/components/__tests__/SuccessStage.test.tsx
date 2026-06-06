import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import { SuccessStage } from '../SuccessStage';
import { useOnboardingStore } from '../../store';
import { TooltipProvider } from '../../../../components/TooltipRegistry';
import { describe, it, expect, beforeEach } from 'vitest';

describe('SuccessStage', () => {
  const renderComponent = () => render(
    <TooltipProvider>
      <SuccessStage />
    </TooltipProvider>
  );

  beforeEach(() => {
    useOnboardingStore.setState({
      startResult: { message: 'Success launch message' },
    });
  });

  it('renders success message', () => {
    renderComponent();
    expect(screen.getByText(/You're Live/i)).toBeInTheDocument();
    expect(screen.getByText('Success launch message')).toBeInTheDocument();
  });

  it('renders dashboard link', () => {
    renderComponent();
    expect(screen.getByRole('link', { name: /Enter Dashboard/i })).toBeInTheDocument();
  });
});
