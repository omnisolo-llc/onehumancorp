import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import { afterEach } from 'vitest';
import ZeroClickBuilderPage from './page';

// Mock Next.js router
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

vi.mock('../../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc">Powered by OHC</div>,
}));

describe('ZeroClickBuilderPage', () => {
  beforeEach(() => {
    // Reset fetch mock
    global.fetch = vi.fn() as unknown as typeof fetch;
    localStorage.clear();
    vi.stubEnv('NODE_ENV', 'test');
  });

  afterEach(() => {
    vi.unstubAllEnvs();
  });

  it('renders the initial form', () => {
    render(<ZeroClickBuilderPage />);
    expect(screen.getByText('Tell us about your business')).toBeInTheDocument();
    expect(screen.getByPlaceholderText(/e.g. I am a home baker in Austin selling custom vegan cakes./i)).toBeInTheDocument();
    const buttons = screen.getAllByRole('button');
    const submitBtn = buttons[buttons.length - 1];
    expect(submitBtn).toBeDisabled();
  });

  it('enables the button when prompt is entered', () => {
    render(<ZeroClickBuilderPage />);
    const input = screen.getByPlaceholderText(/e.g. I am a home baker in Austin selling custom vegan cakes./i);
    fireEvent.change(input, { target: { value: 'I sell custom sneakers' } });

    const buttons = screen.getAllByRole('button');
    const submitBtn = buttons[buttons.length - 1];
    expect(submitBtn).toBeEnabled();
  });

  it('submits the form and displays the result', async () => {
    global.fetch = vi.fn().mockImplementation((url: string) => {
      if (url === '/api/v1/billing/my-plan') {
        return Promise.resolve({ ok: true, json: async () => ({ current_plan: 'free' }) });
      }
      if (url === '/api/v1/onboarding/chat') {
        return Promise.resolve({
          ok: true,
          json: async () => ({
            reply: 'Got it. I will build a sneakers store.',
            is_complete: true,
            intake_data: {
              business_name: 'Custom Sneakers Store',
              business_type: 'Retail',
              categories: ['physical'],
              initial_products: [{ name: 'Sneakers', price: '100' }]
            }
          }),
        });
      }
      if (url === '/api/v1/onboarding/start') {
        return Promise.resolve({
          ok: true,
          json: async () => ({ organization_id: 'org_123', user_id: 'user_123' }),
        });
      }
      return Promise.resolve({ ok: false, json: async () => ({}) });
    });

    render(<ZeroClickBuilderPage />);

    const input = screen.getByPlaceholderText(/e.g. I am a home baker in Austin selling custom vegan cakes./i);
    fireEvent.change(input, { target: { value: 'I sell custom sneakers' } });

    const buttons = screen.getAllByRole('button');
    const submitBtn = buttons[buttons.length - 1];
    fireEvent.click(submitBtn);

    // Wait for the result to appear
    await waitFor(() => {
      expect(screen.getByText('Your business is live!')).toBeInTheDocument();
    }, { timeout: 3000 });

    expect(screen.getByTitle('Live Storefront Preview')).toBeInTheDocument();
    const launch = screen.getByRole('button', { name: /Launch My Store/i });
    expect(launch).toBeInTheDocument();
    const startCall = vi.mocked(global.fetch).mock.calls.find(([url]) => url === '/api/v1/onboarding/start');
    const startBody = JSON.parse(String(startCall?.[1]?.body));
    expect(startBody.admin_name).toBeUndefined();
    expect(startBody.admin_email).toBeUndefined();
    expect(startBody.admin_password).toBeUndefined();

    fireEvent.click(launch);
    expect(localStorage.getItem('business_display_name')).toBeNull();
    expect(localStorage.getItem('business_display_name')).toBeNull();
    expect(localStorage.getItem('user_display_name')).toBeNull();
  });

  it('renders Powered by OHC branding', () => {
    render(<ZeroClickBuilderPage />);
    const texts = screen.getAllByText(/Powered by OHC/i);
    expect(texts.length).toBeGreaterThan(0);
  });

  it('renders the PoweredByOHC component', () => {
    render(<ZeroClickBuilderPage />);
    const components = screen.getAllByTestId('powered-by-ohc');
    expect(components.length).toBeGreaterThan(0);
  });
});
