import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import TaskDAGViewer from '../src/components/orchestration/TaskDAGViewer';
import TeammateMeshConsole from '../src/components/orchestration/TeammateMeshConsole';
import SwarmOverview from '../src/components/orchestration/SwarmOverview';
import AutoDreamPipelineWidget from '../src/components/orchestration/AutoDreamPipelineWidget';

describe('Swarm Orchestration UI Components', () => {
  test('SwarmOverview renders correctly', () => {
    render(<SwarmOverview />);
    expect(screen.getByText('Swarm Overview')).toBeInTheDocument();
    expect(screen.getByTestId('active-agents')).toHaveTextContent('12');
    expect(screen.getByTestId('completed-tasks')).toHaveTextContent('145');
  });

  test('AutoDreamPipelineWidget renders correctly', () => {
    render(<AutoDreamPipelineWidget />);
    expect(screen.getByTestId('autodream-pipeline')).toBeInTheDocument();
    expect(screen.getByText('AutoDream Pipeline Stream')).toBeInTheDocument();
    expect(screen.getByText('Extract')).toBeInTheDocument();
    expect(screen.getByText('Analyze')).toBeInTheDocument();
  });

  test('TaskDAGViewer fetches and renders tasks', async () => {
    global.fetch = jest.fn(() =>
      Promise.resolve({
        json: () => Promise.resolve([
          { id: '1', title: 'Task 1', status: 'PENDING' },
          { id: '2', title: 'Task 2', status: 'COMPLETED' }
        ]),
      })
    ) as jest.Mock;

    render(<TaskDAGViewer />);
    expect(screen.getByText('Loading tasks...')).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText('Task 1')).toBeInTheDocument();
      expect(screen.getByText('PENDING')).toBeInTheDocument();
    });

    expect(screen.getByText('Task 2')).toBeInTheDocument();
    expect(screen.getByText('COMPLETED')).toBeInTheDocument();
  });

  test('TeammateMeshConsole connects to websocket', () => {
    const mockWebSocket = {
      onmessage: null,
      close: jest.fn(),
    };
    global.WebSocket = jest.fn(() => mockWebSocket) as any;

    render(<TeammateMeshConsole />);
    expect(screen.getByText('Teammate Mesh Console')).toBeInTheDocument();
    expect(global.WebSocket).toHaveBeenCalledWith('ws://localhost:8080/api/mesh/stream');
  });
});
