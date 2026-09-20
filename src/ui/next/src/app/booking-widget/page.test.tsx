import { render, screen } from '@testing-library/react';
import React from 'react';
import BookingWidgetBuilder from './page';

vi.mock("next/navigation", () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('BookingWidgetBuilder', () => {
    it('renders the booking widget builder with deposit and travel buffer', () => {
        render(<BookingWidgetBuilder />);
        expect(screen.getByText('Deposit Required ($)')).toBeDefined();
        expect(screen.getByText('Need Travel Time? (Dynamic Buffer)')).toBeDefined();
    });
});
