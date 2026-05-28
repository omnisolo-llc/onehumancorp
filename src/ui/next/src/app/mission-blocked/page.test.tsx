import React from 'react';
import { render, screen } from '@testing-library/react';
import { expect, test, describe } from 'vitest';
import MissionBlockedPage from './page';

describe('MissionBlockedPage', () => {
  test('renders Setup Required heading', () => {
    render(<MissionBlockedPage />);
    expect(screen.getByText('Setup Required')).toBeInTheDocument();
  });

  test('renders non-technical descriptions', () => {
    render(<MissionBlockedPage />);
    expect(screen.getByText(/Your AI helpers are ready to get to work/i)).toBeInTheDocument();
    expect(screen.getByText(/Mission Paused/i)).toBeInTheDocument();
    expect(screen.getByText(/Pending storage connection/i)).toBeInTheDocument();
  });

  test('does not render technical jargon', () => {
    const { container } = render(<MissionBlockedPage />);
    expect(container.textContent).not.toMatch(/PostgreSQL/i);
    expect(container.textContent).not.toMatch(/agent_missions/i);
    expect(container.textContent).not.toMatch(/database/i);
  });
});
