import '@testing-library/jest-dom';
import React from 'react';
import { render, screen } from '@testing-library/react';
import GettingStartedPage from './page';
import { describe, it, expect, vi } from 'vitest';

vi.mock('../../../components/Walkthrough', () => ({
  WalkthroughTarget: ({ id, children }: any) => <div data-testid={`target-${id}`}>{children}</div>
}));

vi.mock('../../../components/help', () => ({
  useWalkthrough: () => ({
    startWalkthrough: vi.fn()
  })
}));

describe('GettingStartedPage', () => {
  it('renders correctly', () => {
    render(<GettingStartedPage />);
    expect(screen.getByText('Getting Started with Your Store')).toBeInTheDocument();
  });
});
