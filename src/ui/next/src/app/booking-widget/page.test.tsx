import { render, screen, fireEvent } from '@testing-library/react';
import { beforeEach, expect, test, vi } from 'vitest';
import BookingWidgetBuilder from './page';

const { push } = vi.hoisted(() => ({ push: vi.fn() }));
vi.mock('next/navigation', () => ({ useRouter: () => ({ push }) }));

beforeEach(() => { vi.clearAllMocks(); });

test('names the icon-only dashboard link for keyboard and screen-reader navigation', () => {
  render(<BookingWidgetBuilder />);
  expect(screen.getByRole('link', { name: 'Back to Dashboard' })).toHaveAttribute('href', '/dashboard');
});

test('associates configuration labels with the controls that update the preview', () => {
  render(<BookingWidgetBuilder />);
  fireEvent.change(screen.getByRole('textbox', { name: 'Tenant ID' }), { target: { value: 'store & team' } });
  fireEvent.change(screen.getByRole('textbox', { name: 'Service Name' }), { target: { value: 'Kitchen consultation' } });

  expect(screen.getByRole('heading', { name: 'Kitchen consultation' })).toBeVisible();
  expect(screen.getByRole('link', { name: 'Powered by OmniSolo' })).toHaveAttribute(
    'href', '/onboarding?ref=store%20%26%20team&source=booking_widget_preview',
  );
});

test('keeps the service preview connected to the real booking route', () => {
  render(<BookingWidgetBuilder />);
  fireEvent.click(screen.getByRole('button', { name: 'Request a Service' }));
  expect(push).toHaveBeenCalledWith('/booking');
});
