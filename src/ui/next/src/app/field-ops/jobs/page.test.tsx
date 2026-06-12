<<<<<<< HEAD
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
=======
import { render, screen, fireEvent } from '@testing-library/react';
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
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
<<<<<<< HEAD

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
=======
  });

  it('renders the daily roster', () => {
    render(<FieldOpsJobsPage />);
    expect(screen.getByText("Today's Route")).toBeInTheDocument();
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
    expect(screen.getByText('Alice Smith')).toBeInTheDocument();
    expect(screen.getByText('Bob Jones')).toBeInTheDocument();
  });

<<<<<<< HEAD
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
=======
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
>>>>>>> d1af2215 (Fix unhandled updates warning in ChaosReportPage tests (#26923))
    expect(screen.getByText(/"Needs new piping"/)).toBeInTheDocument();
  });
});
