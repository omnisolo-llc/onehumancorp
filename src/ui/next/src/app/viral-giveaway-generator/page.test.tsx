import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import '@testing-library/jest-dom';
import ViralGiveawayGenerator from './page';

describe('ViralGiveawayGenerator', () => {
  it('renders the generator form', () => {
    render(<ViralGiveawayGenerator />);
    expect(screen.getByText('Viral Giveaway Builder')).toBeInTheDocument();
    expect(screen.getByTestId('giveaway-name-input')).toBeInTheDocument();
    expect(screen.getByTestId('prize-input')).toBeInTheDocument();
  });

  it('disables the generate button when fields are empty', () => {
    render(<ViralGiveawayGenerator />);
    const button = screen.getByTestId('generate-button');
    expect(button).toBeDisabled();
  });

  it('enables the generate button when fields are filled', () => {
    render(<ViralGiveawayGenerator />);

    fireEvent.change(screen.getByTestId('giveaway-name-input'), { target: { value: 'Summer Bash' } });
    fireEvent.change(screen.getByTestId('prize-input'), { target: { value: 'Free Ticket' } });

    const button = screen.getByTestId('generate-button');
    expect(button).not.toBeDisabled();
  });

  it('generates a link correctly', () => {
    render(<ViralGiveawayGenerator />);

    fireEvent.change(screen.getByTestId('giveaway-name-input'), { target: { value: 'Summer Bash' } });
    fireEvent.change(screen.getByTestId('prize-input'), { target: { value: 'Free Ticket' } });

    fireEvent.click(screen.getByTestId('generate-button'));

    const generatedLink = screen.getByTestId('generated-link');
    expect(generatedLink).toHaveValue('https://ohc.app/giveaway/summer-bash');
  });
});
