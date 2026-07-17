import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { vi, describe, it, expect, beforeEach } from 'vitest';
import DiscoveryReportPage from './page';

global.fetch = vi.fn();

describe('DiscoveryReportPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders loading state initially', () => {
    (global.fetch as ReturnType<typeof vi.fn>).mockImplementation(() => new Promise(() => {}));
    render(<DiscoveryReportPage />);
    expect(screen.getByText('Loading your report...')).toBeTruthy();
  });

  it('renders empty state when no reports exist', async () => {
    (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValue({
      ok: true,
      json: async () => [],
    });

    render(<DiscoveryReportPage />);

    await waitFor(() => {
      expect(screen.getByText('No Reports Yet')).toBeTruthy();
    });
    expect(screen.getByText(/Your first AI Discovery Report will be generated soon/i)).toBeTruthy();
  });

  it('renders reports correctly', async () => {
    (global.fetch as ReturnType<typeof vi.fn>).mockResolvedValue({
      ok: true,
      json: async () => [
        {
          id: '123',
          month: 'June 2026',
          plain_language_summary: 'ChatGPT recommended your handyman services 15 times this week to locals in your area.',
          metrics: { chatgpt_recommendations: 15, gemini_recommendations: 4 }
        }
      ],
    });

    render(<DiscoveryReportPage />);

    await waitFor(() => {
      expect(screen.getByText('June 2026')).toBeTruthy();
    });

    expect(screen.getByText('ChatGPT recommended your handyman services 15 times this week to locals in your area.')).toBeTruthy();
    expect(screen.getByText('chatgpt recommendations')).toBeTruthy();
    expect(screen.getByText('15')).toBeTruthy();
  });
});
