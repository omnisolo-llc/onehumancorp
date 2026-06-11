import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ExitIntentGeneratorPage from './page';

// Mock AppShell to just render children
vi.mock('../components/AppShell', () => ({
  AppShell: ({ children }: { children: React.ReactNode }) => <div data-testid="app-shell">{children}</div>,
}));

describe('ExitIntentGeneratorPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Storage.prototype.getItem = vi.fn((key) => {
      if (key === 'tenant_id') return 'test-tenant';
      if (key === 'tier') return 'Free';
      return null;
    });
  });

  it('renders the exit intent generator with Powered by OHC by default', () => {
    render(<ExitIntentGeneratorPage />);

    // Check main title
    expect(screen.getByText('Configure Exit Intent Pop-up')).toBeDefined();

    // Check preview elements
    expect(screen.getByText("Wait! Don't leave yet.")).toBeDefined();

    // Check for Powered by OHC branding in preview
    const brandingElements = screen.getAllByText(/Powered by OHC/i);
    expect(brandingElements.length).toBeGreaterThan(0);

    // Check if the checkbox is disabled (since Free tier)
    const checkbox = screen.getByLabelText(/Remove "Powered by OHC" Badge/i);
    expect((checkbox as HTMLInputElement).disabled).toBe(true);
  });
});
