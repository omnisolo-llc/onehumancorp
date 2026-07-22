import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import AgentTerminalPage from './page';

describe('AgentTerminalPage', () => {
  beforeEach(() => {
    global.fetch = vi.fn((url: string) => {
      if (url.includes('/api/v1/payments/terminal/backend')) {
        return Promise.resolve({
          json: () => Promise.resolve({ backend: 'local' }),
          ok: true,
        });
      }
      if (url.includes('/api/v1/payments/terminal/session/start')) {
        return Promise.resolve({
          json: () => Promise.resolve({ output: 'Mocked output' }),
          ok: true,
        });
      }
      return Promise.resolve({ ok: true });
    }) as any;
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders the terminal and switches backends', async () => {
    render(<AgentTerminalPage />);
    expect(screen.getByText('Assistant-First Shell')).toBeInTheDocument();
    const select = screen.getByLabelText('Terminal Backend:');
    fireEvent.change(select, { target: { value: 'docker' } });
    expect(screen.getByText('[System] Switched to docker backend.')).toBeInTheDocument();

    fireEvent.change(select, { target: { value: 'ssh' } });
    expect(screen.getByText('[System] Switched to ssh backend.')).toBeInTheDocument();

    fireEvent.change(select, { target: { value: 'singularity' } });
    expect(screen.getByText('[System] Switched to singularity backend.')).toBeInTheDocument();

    fireEvent.change(select, { target: { value: 'modal' } });
    expect(screen.getByText('[System] Switched to modal backend.')).toBeInTheDocument();

    fireEvent.change(select, { target: { value: 'daytona' } });
    expect(screen.getByText('[System] Switched to daytona backend.')).toBeInTheDocument();

    fireEvent.change(select, { target: { value: 'vercal_sandbox' } });
    expect(screen.getByText('[System] Switched to vercal_sandbox backend.')).toBeInTheDocument();
  });

  it('submits a command', async () => {
    render(<AgentTerminalPage />);
    const input = screen.getByPlaceholderText('Enter command (e.g. echo hello)...');
    const button = screen.getByRole('button', { name: 'Submit' });
    fireEvent.change(input, { target: { value: 'test command' } });
    fireEvent.click(button);
    await waitFor(() => {
      expect(screen.getByText('$ test command')).toBeInTheDocument();
      expect(screen.getByText('Mocked output')).toBeInTheDocument();
    });
  });
});
