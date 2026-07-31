import { render, screen, fireEvent } from '@testing-library/react';
import GrowthPage from './page';
import { vi, describe, it, expect, beforeEach } from 'vitest';

vi.mock('../components/AppShell', () => ({
  AppShell: ({ children }: any) => <div data-testid="app-shell">{children}</div>,
}));

describe('GrowthPage', () => {
  beforeEach(() => {
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn(),
      },
    });
  });

  it('renders the referral section', () => {
    render(<GrowthPage />);
    expect(screen.getByText('One-Tap Referral')).toBeInTheDocument();
    expect(screen.getByText('Copy Link')).toBeInTheDocument();
  });

  it('handles link copy', () => {
    render(<GrowthPage />);
    const copyButton = screen.getByText('Copy Link');
    fireEvent.click(copyButton);
    expect(navigator.clipboard.writeText).toHaveBeenCalledWith('https://ohc.com/join?ref=YOUR_BUSINESS');
    expect(screen.getByText('Copied!')).toBeInTheDocument();
  });

  it('renders milestones', () => {
    render(<GrowthPage />);
    expect(screen.getByText('Milestones')).toBeInTheDocument();
    expect(screen.getByText('10')).toBeInTheDocument();
    expect(screen.getByText('50')).toBeInTheDocument();
    expect(screen.getByText('100')).toBeInTheDocument();
  });
});
