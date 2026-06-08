import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import RalphLoopDashboard from './page';

global.fetch = vi.fn();

describe('RalphLoopDashboard', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders correctly and handles start task', async () => {
    (global.fetch as ReturnType<typeof vi.fn>).mockImplementation((url) => {
      if (url === '/api/agents/ralph/progress') {
        return Promise.resolve({
          ok: false,
          status: 404,
          json: () => Promise.resolve({}),
        });
      }
      if (url === '/api/agents/ralph/start') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({ success: true }),
        });
      }
      return Promise.resolve({ ok: false });
    });

    render(<RalphLoopDashboard />);

    expect(screen.getByText('Ralph Loop Agent CLI')).toBeInTheDocument();

    const textarea = screen.getByPlaceholderText('Describe the large task you want the agent to accomplish...');
    fireEvent.change(textarea, { target: { value: 'Build a new app' } });

    const startButton = screen.getByText('Start Ralph Agent');
    fireEvent.click(startButton);

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/agents/ralph/start', expect.anything());
    });
  });

  it('displays progress', async () => {
    (global.fetch as ReturnType<typeof vi.fn>).mockImplementation((url) => {
      if (url === '/api/agents/ralph/progress') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({
            task_description: 'Test Task',
            features: [
              { name: 'Feature 1', status: 'completed' },
              { name: 'Feature 2', status: 'pending' },
            ],
            current_feature_index: 1,
            notes: ['Note 1'],
            is_complete: false,
          }),
        });
      }
      return Promise.resolve({ ok: false });
    });

    render(<RalphLoopDashboard />);

    await waitFor(() => {
      expect(screen.getByText('Current Task Progress')).toBeInTheDocument();
      expect(screen.getByText('Test Task')).toBeInTheDocument();
      expect(screen.getByText('Feature 1')).toBeInTheDocument();
      expect(screen.getByText('Feature 2')).toBeInTheDocument();
      expect(screen.getByText('> Note 1')).toBeInTheDocument();
    });
  });
});
