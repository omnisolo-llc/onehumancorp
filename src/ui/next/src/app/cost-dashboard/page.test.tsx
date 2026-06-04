import { render, screen } from '@testing-library/react';
import CostDashboardPage from './page';
import React from 'react';
import { describe, it, expect, vi, beforeEach } from 'vitest';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

global.fetch = vi.fn();

describe('CostDashboardPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders loading state initially', () => {
    (global.fetch as any).mockImplementationOnce(() => new Promise(() => {}));
    render(<CostDashboardPage />);
    expect(screen.getByText('Loading...')).toBeDefined();
  });
});
