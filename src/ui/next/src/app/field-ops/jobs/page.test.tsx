import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import FieldOpsJobsPage from './page';

vi.mock('../../../lib/sync/SyncManager', () => ({
  SyncManager: {
    getInstance: vi.fn(() => ({
      enqueue: vi.fn(),
    })),
  },
}));

vi.mock('../../../lib/powersync/PowerSyncProvider', () => ({
  PowerSyncProvider: ({ children }: any) => <div data-testid="powersync-provider">{children}</div>,
  isPowerSyncSupportedForLocation: () => true
}));
vi.mock('../../../lib/powersync/db', () => ({
  getPowerSyncDB: vi.fn(() => Promise.resolve({
    execute: vi.fn()
  }))
}));

vi.mock('@powersync/react', () => ({
  useQuery: vi.fn(() => ({ data: [] })),
  PowerSyncContext: { Provider: ({ children }: any) => <div>{children}</div> }
}));

describe('FieldOpsJobsPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.defineProperty(navigator, 'onLine', { value: true, writable: true });

    global.fetch = vi.fn(() =>
      Promise.resolve({
        json: () => Promise.resolve({
          appointments: [
            {
              id: 'job-1',
              customer_id: 'cust-1',
              customer_name: 'Alice Smith',
              job_template_id: 'job-plumbing',
              job_name: 'Plumbing Repair',
              status: 'Scheduled',
              scheduled_start_time: new Date().toISOString(),
              scheduled_end_time: new Date(Date.now() + 3600000).toISOString(),
              location_address: '123 Main St',
              notes: ''
            },
            {
              id: 'job-2',
              customer_id: 'cust-2',
              customer_name: 'Bob Jones',
              job_template_id: 'job-elec',
              job_name: 'Electrical Inspection',
              status: 'Requested',
              scheduled_start_time: new Date(Date.now() + 7200000).toISOString(),
              scheduled_end_time: new Date(Date.now() + 10800000).toISOString(),
              location_address: '456 Oak Ave',
              notes: ''
            }
          ]
        }),
      })
    ) as any;
  });

  it('renders the daily roster after loading', async () => {
    render(<FieldOpsJobsPage />);
    await waitFor(() => {
      expect(screen.getByText("Today's Route")).toBeInTheDocument();
    });
    expect(screen.getByText('Alice Smith')).toBeInTheDocument();
    expect(screen.getByText('Bob Jones')).toBeInTheDocument();
  });

  it('shows offline indicator when offline', async () => {
    Object.defineProperty(navigator, 'onLine', { value: false });
    render(<FieldOpsJobsPage />);
    await waitFor(() => {
      expect(screen.getByText(/Offline Mode/)).toBeInTheDocument();
    });
  });

  it('allows state transitions and completing a job', async () => {
    render(<FieldOpsJobsPage />);

    await waitFor(() => {
      expect(screen.getByText('Alice Smith')).toBeInTheDocument();
    });

    const textareas = screen.getAllByPlaceholderText(/E.g., Needs a replacement quote./);
    fireEvent.change(textareas[0], { target: { value: 'Needs new piping' } });

    const headingButtons = screen.getAllByText('Heading to Job');
    fireEvent.click(headingButtons[0]);

    const startWorkButton = await screen.findByText('Start Work');
    fireEvent.click(startWorkButton);

    const completeButton = await screen.findByText('Job Done');
    fireEvent.click(completeButton);

    expect(await screen.findByText('Saved Notes:')).toBeInTheDocument();
    expect(screen.getByText(/"Needs new piping"/)).toBeInTheDocument();
  });
});
