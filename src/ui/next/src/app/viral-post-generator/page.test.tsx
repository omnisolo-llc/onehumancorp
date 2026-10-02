import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ViralPostGeneratorPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(() => ({
    push: vi.fn(),
  })),
}));

describe('ViralPostGeneratorPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn(),
      },
    });

    const localStorageMock = {
      getItem: vi.fn((key) => {
        if (key === 'tenant') return 'test-tenant';
        return null;
      }),
      setItem: vi.fn(),
      clear: vi.fn()
    };
    Object.defineProperty(window, 'localStorage', {
      value: localStorageMock,
      writable: true
    });

    // Mock window.open
    Object.defineProperty(window, 'open', {
        value: vi.fn(),
        writable: true
    });
    global.fetch = vi.fn().mockImplementation((url: string, options?: RequestInit) => {
      if (url === '/api/v1/growth/trial-extension/claim' && options?.method === 'POST') {
        return Promise.resolve({ ok: true, json: async () => ({}) });
      }
      return Promise.resolve({ ok: true, json: async () => ({ current_plan: 'free' }) });
    });
  });

  it('renders correctly', () => {
    render(<ViralPostGeneratorPage />);
    expect(screen.getByText('Social Post Template 🚀')).toBeDefined();
  });

  it('generates a post', () => {
    render(<ViralPostGeneratorPage />);

    const productNameInput = screen.getByPlaceholderText('e.g. Signature Coffee Blend');
    fireEvent.change(productNameInput, { target: { value: 'Test Product' } });

    const keyBenefitInput = screen.getByPlaceholderText('e.g. a bold start to your morning');
    fireEvent.change(keyBenefitInput, { target: { value: 'Testing benefits' } });

    const generateBtn = screen.getByRole('button', { name: 'Generate Post' });
    fireEvent.click(generateBtn);

    expect(screen.getByText(/Test Product/)).toBeDefined();
    expect(screen.getByText(/Testing benefits/)).toBeDefined();
    expect(screen.getAllByText(/Powered by OmniSolo/).length).toBeGreaterThan(0);
  });

  it('copies to clipboard', async () => {
    render(<ViralPostGeneratorPage />);

    const productNameInput = screen.getByPlaceholderText('e.g. Signature Coffee Blend');
    fireEvent.change(productNameInput, { target: { value: 'Test Product' } });

    const keyBenefitInput = screen.getByPlaceholderText('e.g. a bold start to your morning');
    fireEvent.change(keyBenefitInput, { target: { value: 'Testing benefits' } });

    const generateBtn = screen.getByRole('button', { name: 'Generate Post' });
    fireEvent.click(generateBtn);

    const copyBtn = screen.getByRole('button', { name: 'Copy to Clipboard' });
    fireEvent.click(copyBtn);

    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    await waitFor(() => expect(screen.getByText('Copied!')).toBeDefined());
  });

  it('shows paywall when toggling remove branding', () => {
    render(<ViralPostGeneratorPage />);
    const checkbox = screen.getByRole('checkbox');
    fireEvent.click(checkbox);

    expect(screen.getAllByText('Upgrade to Pro').length).toBeGreaterThan(0);
    expect(screen.getByRole('button', { name: 'Check trial availability' })).toBeDefined();
  });

  it('keeps trial activation unavailable without a grant mutation', async () => {
    render(<ViralPostGeneratorPage />);
    const checkbox = screen.getByRole('checkbox');
    fireEvent.click(checkbox);

    const shareBtn = screen.getByRole('button', { name: 'Check trial availability' });
    fireEvent.click(shareBtn);

    expect(window.open).not.toHaveBeenCalled();
    await waitFor(() => expect(screen.getByText(/durable grant is not verified/)).toBeVisible());
    expect(vi.mocked(fetch).mock.calls.some(([, options]) => options?.method === 'POST')).toBe(false);
    expect(screen.getAllByText('Upgrade to Pro').length).toBeGreaterThan(0);
    expect(window.localStorage.setItem).not.toHaveBeenCalled();
  });
  it('shows required input guidance and disables empty or whitespace generation',()=>{
    render(<ViralPostGeneratorPage/>);const button=screen.getByRole('button',{name:'Generate Post'});expect(button).toBeDisabled();expect(screen.getByRole('status',{name:'Post requirements'})).toHaveTextContent(/product name.*key benefit/i);
    fireEvent.change(screen.getByPlaceholderText('e.g. Signature Coffee Blend'),{target:{value:'  '}});fireEvent.change(screen.getByPlaceholderText('e.g. a bold start to your morning'),{target:{value:'Actual benefit'}});expect(button).toBeDisabled();fireEvent.click(button);expect(screen.queryByRole('button',{name:'Copy to Clipboard'})).not.toBeInTheDocument();
    fireEvent.change(screen.getByPlaceholderText('e.g. Signature Coffee Blend'),{target:{value:'Actual product'}});expect(button).toBeEnabled();fireEvent.click(button);expect(screen.getByText(/Introducing the new Actual product/)).toBeInTheDocument();
  });

});
