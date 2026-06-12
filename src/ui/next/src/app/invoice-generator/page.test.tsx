import { render, screen, fireEvent } from '@testing-library/react';
import InvoiceGeneratorPage from './page';

describe('InvoiceGeneratorPage', () => {
  it('renders the branding toggle', () => {
    render(<InvoiceGeneratorPage />);
    const toggle = screen.getByLabelText(/Remove "Powered by OHC" Badge \(Pro\)/i);
    expect(toggle).toBeDefined();
  });
});
