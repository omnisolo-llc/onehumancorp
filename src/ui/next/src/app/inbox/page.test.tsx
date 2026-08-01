import { render, screen } from '@testing-library/react';
import Page from './page';
import { vi } from 'vitest';

vi.mock("@/app/components/AppShell", () => ({
  default: ({ children }: any) => <div data-testid="app-shell">{children}</div>,
  AppShell: ({ children }: any) => <div data-testid="app-shell">{children}</div>,
}));

vi.mock("@/components/TooltipRegistry", () => ({
  useTooltip: () => ({}),
  TooltipProvider: ({ children }: any) => <div>{children}</div>,
  WithTooltip: ({ children }: any) => <div>{children}</div>,
}));

describe('Inbox Page UI (Native Rust Migration)', () => {
  it('renders correctly', () => {
    render(<Page />);
    // Testing the dummy placeholder render
    expect(screen.getByTestId('app-shell')).toBeInTheDocument();
  });
});
