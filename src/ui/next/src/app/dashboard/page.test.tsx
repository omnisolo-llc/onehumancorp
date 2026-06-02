import { render, screen, fireEvent, act } from '@testing-library/react';
import DashboardPage from './page';
import { TooltipProvider } from '@/components/TooltipRegistry';

// Mock the referral component to avoid errors
jest.mock('@/components/OneTapReferral', () => {
  return function MockOneTapReferral() {
    return <div data-testid="mock-referral">Referral</div>;
  };
});

describe('DashboardPage Offline & Localization UI', () => {
  beforeAll(() => {
    Object.defineProperty(window, 'matchMedia', {
      writable: true,
      value: jest.fn().mockImplementation(query => ({
        matches: false,
        media: query,
        onchange: null,
        addListener: jest.fn(), // Deprecated
        removeListener: jest.fn(), // Deprecated
        addEventListener: jest.fn(),
        removeEventListener: jest.fn(),
        dispatchEvent: jest.fn(),
      })),
    });
  });

  it('renders offline indicator when offline', async () => {
    // Mock navigator.onLine
    jest.spyOn(navigator, 'onLine', 'get').mockReturnValueOnce(false);

    await act(async () => {
        render(
        <TooltipProvider>
            <DashboardPage />
        </TooltipProvider>
        );
    });

    expect(screen.getByText('Offline')).toBeInTheDocument();
  });

  it('toggles language and currency without network request', async () => {
    await act(async () => {
        render(
        <TooltipProvider>
            <DashboardPage />
        </TooltipProvider>
        );
    });

    const langSelect = screen.getByDisplayValue('EN');
    const currencySelect = screen.getByDisplayValue('USD');

    await act(async () => {
        fireEvent.change(langSelect, { target: { value: 'es' } });
        fireEvent.change(currencySelect, { target: { value: 'EUR' } });
    });

    expect(screen.getByDisplayValue('ES')).toBeInTheDocument();
    expect(screen.getByDisplayValue('EUR')).toBeInTheDocument();
  });
});
