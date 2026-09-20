import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import BookingWidgetBuilder from './page';
import { vi, describe, it, expect, beforeEach } from 'vitest';
import { useRouter } from 'next/navigation';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(),
}));

describe('BookingWidgetBuilder', () => {
  beforeEach(() => {
    (useRouter as any).mockReturnValue({
      push: vi.fn(),
    });
  });

  it('renders the configuration form and interactions', () => {
    render(<BookingWidgetBuilder />);
    expect(screen.getByText('Tenant ID')).toBeDefined();
    expect(screen.getByText('Service Name')).toBeDefined();

    const tenantInput = screen.getByPlaceholderText('e.g. my-store');
    fireEvent.change(tenantInput, { target: { value: 'new-tenant' } });

    const serviceInput = screen.getByPlaceholderText('e.g. Service Consultation');
    fireEvent.change(serviceInput, { target: { value: 'New Service' } });

    expect(screen.getByText('Get Widget')).toBeDefined();
    fireEvent.click(screen.getByText('Get Widget'));
    expect(screen.getByText('Embed Booking Widget')).toBeDefined();
  });
});
