import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import BridgeStatusWidget from '../src/components/orchestration/BridgeStatusWidget';

describe('BridgeStatusWidget', () => {
  beforeEach(() => {
    global.fetch = jest.fn(() =>
      Promise.resolve({
        json: () => Promise.resolve({
          status: {
            'remote-org-1': 'ACTIVE',
            'remote-org-2': 'INACTIVE'
          }
        }),
      })
    ) as jest.Mock;
  });

  afterEach(() => {
    jest.resetAllMocks();
  });

  test('renders bridge status correctly', async () => {
    render(<BridgeStatusWidget />);

    expect(screen.getByText('Universal Mesh Bridge')).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText('Org: remote-org-1')).toBeInTheDocument();
      expect(screen.getByText('ACTIVE')).toBeInTheDocument();

      expect(screen.getByText('Org: remote-org-2')).toBeInTheDocument();
      expect(screen.getByText('INACTIVE')).toBeInTheDocument();
    });
  });
});
