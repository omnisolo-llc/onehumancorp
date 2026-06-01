import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import { GlassCard, GlassButton, GlassInput, StepProgress } from './index';
import '@testing-library/jest-dom';

describe('Glass Components', () => {
  describe('GlassCard', () => {
    it('renders children and applies custom className', () => {
      render(<GlassCard className="custom-card">Test Content</GlassCard>);
      const card = screen.getByText('Test Content');
      expect(card).toBeInTheDocument();
      expect(card).toHaveClass('mac-glass-container');
      expect(card).toHaveClass('custom-card');
    });
  });

  describe('GlassButton', () => {
    it('renders children and applies variant styles', () => {
      render(<GlassButton variant="primary">Click Me</GlassButton>);
      const button = screen.getByRole('button', { name: /Click Me/i });
      expect(button).toHaveClass('bg-[#0066FF]');
    });

    it('shows loading state', () => {
      render(<GlassButton isLoading>Click Me</GlassButton>);
      expect(screen.getByText('Processing...')).toBeInTheDocument();
      expect(screen.getByRole('button')).toBeDisabled();
    });
  });

  describe('GlassInput', () => {
    it('renders label and input', () => {
      render(<GlassInput label="Business Name" placeholder="Enter name" />);
      expect(screen.getByText('Business Name')).toBeInTheDocument();
      expect(screen.getByPlaceholderText('Enter name')).toBeInTheDocument();
    });

    it('renders as textarea when specified', () => {
      render(<GlassInput isTextArea placeholder="Describe" />);
      const textarea = screen.getByPlaceholderText('Describe');
      expect(textarea.tagName).toBe('TEXTAREA');
    });
  });

  describe('StepProgress', () => {
    it('renders progress correctly', () => {
      render(<StepProgress currentStep={1} totalSteps={3} />);
      expect(screen.getByText('Step 1 of 3')).toBeInTheDocument();
      expect(screen.getByText('33% Complete')).toBeInTheDocument();
    });
  });
});
