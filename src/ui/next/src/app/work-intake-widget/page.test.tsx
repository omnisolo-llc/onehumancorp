import { render, screen, fireEvent } from '@testing-library/react';
import { expect, test, describe, vi, beforeEach } from 'vitest';
import WorkIntakeWidgetPage from './page';
import { useRouter } from 'next/navigation';

// Mock the AppShell component
vi.mock('../components/AppShell', () => ({
  AppShell: ({ children }: { children: React.ReactNode }) => <div data-testid="app-shell">{children}</div>,
}));

// Mock Next.js router
vi.mock('next/navigation', () => ({
  useRouter: vi.fn(),
}));

// Mock clipboard
Object.assign(navigator, {
  clipboard: {
    writeText: vi.fn().mockImplementation(() => Promise.resolve()),
  },
});

describe('WorkIntakeWidgetPage', () => {
  const mockPush = vi.fn();

  beforeEach(() => {
    (useRouter as unknown as ReturnType<typeof vi.fn>).mockReturnValue({ push: mockPush });
    mockPush.mockClear();
    (navigator.clipboard.writeText as ReturnType<typeof vi.fn>).mockClear();
  });

  test('renders the page and initial state correctly', () => {
    render(<WorkIntakeWidgetPage />);

    expect(screen.getByTestId('app-shell')).toBeInTheDocument();
    expect(screen.getByText('Widget Configuration')).toBeInTheDocument();
    expect(screen.getByText('Live Preview')).toBeInTheDocument();

    // Check initial values
    const tenantInput = screen.getByTestId('input-tenant-id') as HTMLInputElement;
    expect(tenantInput.value).toBe('demo-tenant');

    const titleInput = screen.getByTestId('input-form-title') as HTMLInputElement;
    expect(titleInput.value).toBe('Contact Us');

    const colorInput = screen.getByTestId('input-button-color') as HTMLInputElement;
    expect(colorInput.value).toBe('#0066ff'); // HTML color inputs normalize to lowercase hex
  });

  test('shows soft paywall when attempting to remove branding', () => {
    render(<WorkIntakeWidgetPage />);

    const removeBrandingCheckbox = screen.getByTestId('input-remove-branding');
    fireEvent.click(removeBrandingCheckbox);

    expect(screen.getByTestId('paywall-modal')).toBeInTheDocument();
    expect(screen.getAllByText('Upgrade to Pro')[0]).toBeInTheDocument();
  });

  test('can close the soft paywall modal', () => {
    render(<WorkIntakeWidgetPage />);

    const removeBrandingCheckbox = screen.getByTestId('input-remove-branding');
    fireEvent.click(removeBrandingCheckbox);

    const closeBtn = screen.getByTestId('btn-close-paywall');
    fireEvent.click(closeBtn);

    expect(screen.queryByTestId('paywall-modal')).not.toBeInTheDocument();
  });

  test('navigates to pricing when clicking upgrade in paywall', () => {
    render(<WorkIntakeWidgetPage />);

    const removeBrandingCheckbox = screen.getByTestId('input-remove-branding');
    fireEvent.click(removeBrandingCheckbox);

    const upgradeBtn = screen.getByTestId('btn-upgrade-pro');
    fireEvent.click(upgradeBtn);

    expect(mockPush).toHaveBeenCalledWith('/pricing');
  });

  test('updates embed code when title is changed', () => {
    render(<WorkIntakeWidgetPage />);

    const titleInput = screen.getByTestId('input-form-title');
    fireEvent.change(titleInput, { target: { value: 'Get in Touch' } });

    const getCodeBtn = screen.getByTestId('btn-get-code');
    fireEvent.click(getCodeBtn);

    const codeBlock = screen.getByTestId('embed-code-block');
    expect(codeBlock.textContent).toContain('title=Get%20in%20Touch');
  });

  test('updates embed code when tenant ID is changed', () => {
    render(<WorkIntakeWidgetPage />);

    const tenantInput = screen.getByTestId('input-tenant-id');
    fireEvent.change(tenantInput, { target: { value: 'custom-tenant-123' } });

    const getCodeBtn = screen.getByTestId('btn-get-code');
    fireEvent.click(getCodeBtn);

    const codeBlock = screen.getByTestId('embed-code-block');
    expect(codeBlock.textContent).toContain('tenant=custom-tenant-123');
    expect(codeBlock.textContent).toContain('ref=custom-tenant-123');
  });

  test('updates embed code when color is changed', () => {
    render(<WorkIntakeWidgetPage />);

    const colorInput = screen.getByTestId('input-button-color');
    fireEvent.change(colorInput, { target: { value: '#ff0000' } });

    const getCodeBtn = screen.getByTestId('btn-get-code');
    fireEvent.click(getCodeBtn);

    const codeBlock = screen.getByTestId('embed-code-block');
    expect(codeBlock.textContent).toContain('color=%23ff0000');
  });

  test('copies embed code to clipboard when button is clicked in modal', () => {
    render(<WorkIntakeWidgetPage />);

    const getCodeBtn = screen.getByTestId('btn-get-code');
    fireEvent.click(getCodeBtn);

    const copyButton = screen.getByTestId('btn-copy-code');
    fireEvent.click(copyButton);

    expect(navigator.clipboard.writeText).toHaveBeenCalled();
  });
});
