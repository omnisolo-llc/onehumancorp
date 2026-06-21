import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import ProposalGeneratorPage from './page';
import { describe, it, expect, beforeEach, vi } from 'vitest';

describe('ProposalGeneratorPage', () => {
  beforeEach(() => {
    localStorage.clear();
  });

  it('renders initial state correctly', () => {
    render(<ProposalGeneratorPage />);
    expect(screen.getByText('Create Professional Proposal')).toBeTruthy();
  });

  it('generates a shareable proposal link on valid input', async () => {
    render(<ProposalGeneratorPage />);

    fireEvent.change(screen.getByPlaceholderText(/e.g. Acme Corp/i), { target: { value: 'Test Client' } });
    fireEvent.change(screen.getByPlaceholderText(/e.g. Website Redesign/i), { target: { value: 'Test Scope' } });
    fireEvent.change(screen.getByPlaceholderText(/e.g. 2500.00/i), { target: { value: '1000' } });
    fireEvent.change(screen.getByPlaceholderText(/e.g. 4-6 Weeks/i), { target: { value: '2 Weeks' } });

    const generateBtn = screen.getByText('Generate Shareable Proposal');
    fireEvent.click(generateBtn);

    await waitFor(() => {
      expect(screen.getByText('Your Proposal is Ready!')).toBeTruthy();
    });
  });
});
