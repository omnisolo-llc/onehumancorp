import { render, screen, waitFor } from '@testing-library/react';
import ProposalViewPage from './page';
import { describe, it, expect, vi, beforeEach } from 'vitest';

// Mock the next/navigation useSearchParams
vi.mock('next/navigation', () => {
  const data = {
    tenant: 'test-tenant',
    clientName: 'Test Client',
    projectScope: 'Test Scope',
    amount: '1000',
    timeline: '2 Weeks'
  };
  const utf8Encoded = encodeURIComponent(JSON.stringify(data));
  const base64Str = btoa(unescape(utf8Encoded));
  const base64UrlStr = base64Str.replace(/\+/g, '-').replace(/\//g, '_').replace(/=/g, '');

  return {
    useSearchParams: () => ({
      get: (key: string) => {
        if (key === 'data') return base64UrlStr;
        return null;
      }
    })
  };
});

describe('ProposalViewPage', () => {
  it('renders correctly with decoded data and includes Powered by OHC watermark', async () => {
    render(<ProposalViewPage />);

    await waitFor(() => {
      expect(screen.getByText('Test Client')).toBeTruthy();
      expect(screen.getByText('Test Scope')).toBeTruthy();
      expect(screen.getByText('2 Weeks')).toBeTruthy();
      expect(screen.getByText('$1000.00')).toBeTruthy();

      // Verify Powered by OHC loop logic exists
      expect(screen.getByText('⚡ Powered by OHC')).toBeTruthy();
      expect(screen.getByText('Create your own professional proposals for free →')).toBeTruthy();
    });
  });
});
