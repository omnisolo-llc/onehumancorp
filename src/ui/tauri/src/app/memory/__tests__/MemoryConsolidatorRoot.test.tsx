import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import MemoryConsolidatorRoot from '../MemoryConsolidatorRoot';
import { invoke } from '@tauri-apps/api/core';

jest.mock('@tauri-apps/api/core', () => ({
    invoke: jest.fn()
}));

describe('MemoryConsolidatorRoot', () => {
    beforeEach(() => {
        (invoke as jest.Mock).mockImplementation((cmd) => {
            if (cmd === 'api_memory_get_metrics') {
                return Promise.resolve({
                    total_records: 100,
                    active_conflicts: 5,
                    pending_prunes: 10,
                    resolved_anomalies: 20
                });
            }
            if (cmd === 'api_memory_get_records') {
                return Promise.resolve([
                    { id: '1', context: 'Test fact', department: 'Sales', confidence: 0.9, timestamp: '2026' }
                ]);
            }
            return Promise.resolve();
        });
    });

    it('renders and fetches data', async () => {
        render(<MemoryConsolidatorRoot />);

        await waitFor(() => {
            expect(screen.getByText('Core Memory Consolidator')).toBeInTheDocument();
            expect(screen.getByText('100')).toBeInTheDocument(); // total records
            expect(screen.getByText('Test fact')).toBeInTheDocument();
        });
    });
});
