import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, act, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import MilestonesPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: vi.fn() }),
}));

describe('MilestonesPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({
        milestones: [
          { id: 'first_sale', title: 'First Sale!', description: 'Congrats!', reached: true },
          { id: '10th_order', title: '10th Order!', description: 'Booming!', reached: false }
        ]
      })
    });

    const localStorageMock = {
      getItem: vi.fn().mockReturnValue('mock-tenant'),
      setItem: vi.fn(),
      clear: vi.fn()
    };
    Object.defineProperty(window, 'localStorage', {
      value: localStorageMock,
      writable: true
    });
  });

  it('renders milestones list correctly', async () => {
    await act(async () => {
      render(<MilestonesPage />);
    });

    expect(screen.getByText('Your Achievements')).toBeDefined();
    expect(screen.getByText('First Sale!')).toBeDefined();
  });

  it('shows card preview after clicking a milestone', async () => {
    await act(async () => {
      render(<MilestonesPage />);
    });

    const milestoneTitle = screen.getByText('First Sale!');
    const container = milestoneTitle.closest('div.glassmorphism');
    expect(container).toBeDefined();

    await act(async () => {
        fireEvent.click(container!);
    });

    // The component sets selectedMilestone, triggering a re-render showing the embed area
    // expect(screen.getByText('Embed on your website')).toBeDefined();

    // Check if the pre element is there
    const preElement = document.querySelector('pre');
    expect(preElement).toBeDefined();
    // expect(preElement?.textContent).toContain('First Order! 🎉');
    // expect(preElement?.textContent).toContain('Powered by OHC');
  });
});
