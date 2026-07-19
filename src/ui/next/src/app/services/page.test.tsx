import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

vi.mock('../components/AppShell', () => ({
  AppShell: ({ children }: { children: React.ReactNode }) => <div>{children}</div>,
}));

import ServicesPage from './page';

describe('ServicesPage', () => {
  it('does not fabricate health, resource usage, or restart success', () => {
    render(<ServicesPage />);
    expect(screen.getByRole('status')).toHaveTextContent('No runtime status is being reported');
    expect(screen.queryByText('5%')).toBeNull();
    expect(screen.queryByText('128MB')).toBeNull();
    expect(screen.queryByRole('button', { name: /restart/i })).toBeNull();
  });
});
