import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import FieldOpsJobsPage from './page';

vi.mock('../../../lib/sync/SyncManager', () => ({
  SyncManager: {
    getInstance: vi.fn(() => ({
      enqueue: vi.fn(),
    })),
  },
}));

describe('FieldOpsJobsPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.defineProperty(navigator, 'onLine', { value: true, writable: true });
  });

  it('renders the daily roster', () => {
    render(<FieldOpsJobsPage />);
    expect(screen.getByText("Today's Route")).toBeInTheDocument();
    expect(screen.getByText('Alice Smith')).toBeInTheDocument();
    expect(screen.getByText('Bob Jones')).toBeInTheDocument();
  });

  it('shows offline indicator when offline', () => {
    Object.defineProperty(navigator, 'onLine', { value: false });
    render(<FieldOpsJobsPage />);
    expect(screen.getByText(/Offline Mode/)).toBeInTheDocument();
  });

  it('allows adding notes and marking a job as complete', () => {
    render(<FieldOpsJobsPage />);

    const textareas = screen.getAllByPlaceholderText(/E.g., Fixed the leak/);
    fireEvent.change(textareas[0], { target: { value: 'Needs new piping' } });

    const completeButtons = screen.getAllByText('Complete Job');
    fireEvent.click(completeButtons[0]);

    expect(screen.getByText('Saved Notes:')).toBeInTheDocument();
    expect(screen.getByText(/"Needs new piping"/)).toBeInTheDocument();
  });
});
