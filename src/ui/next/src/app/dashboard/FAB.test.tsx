import { render, screen, fireEvent } from '@testing-library/react';
import { FloatingActionButton } from './FAB';
import { expect, test, describe } from 'vitest';

describe('FloatingActionButton UI interactions', () => {
  test('verifies button styling matches macOS-style Translucent Glass material', () => {
    render(<FloatingActionButton />);
    const mainButton = screen.getByRole('button');

    // Check baseline class presence
    expect(mainButton.className).toContain('backdrop-blur-[30px]');
    expect(mainButton.className).toContain('backdrop-saturate-[2.1]');
    expect(mainButton.className).toContain('bg-white/65');
  });

  test('verifies links are present after clicking the button', () => {
    render(<FloatingActionButton />);
    const mainButton = screen.getByRole('button');
    fireEvent.click(mainButton);

    // Check links display
    const snapReceiptLink = screen.getByTestId('snap-receipt-fab');
    expect(snapReceiptLink).toBeInTheDocument();
  });
});
