import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import DeliveryTrackingPage from './page';

// Mock fetch
global.fetch = jest.fn((url) => {
    if (url.includes('/api/v1/delivery/update')) {
        return Promise.resolve({
            json: () => Promise.resolve({
                id: "stop_1",
                status: "out_for_delivery"
            })
        });
    }
    return Promise.resolve({
        json: () => Promise.resolve({
            id: "dummy-route",
            organization_id: "system",
            driver_id: "auto_driver",
            status: "planning",
            stops: [
                {
                    id: "stop_1",
                    order_id: "order_1",
                    address: "123 Test St",
                    status: "pending",
                    eta_ms: 1717200000000
                }
            ]
        }),
    });
}) as jest.Mock;

describe('DeliveryTrackingPage', () => {
    it('renders the Next Stop card and Start Leg button', async () => {
        render(<DeliveryTrackingPage />);

        await waitFor(() => {
            expect(screen.getByText('Delivery Dispatch')).toBeInTheDocument();
            expect(screen.getByText('123 Test St')).toBeInTheDocument();
            expect(screen.getByText('Start Leg')).toBeInTheDocument();
        });
    });

    it('can update stop status', async () => {
        const user = userEvent.setup();
        render(<DeliveryTrackingPage />);

        let startBtn: HTMLElement;
        await waitFor(() => {
            startBtn = screen.getByText('Start Leg');
            expect(startBtn).toBeInTheDocument();
        });

        await user.click(startBtn!);

        await waitFor(() => {
            expect(global.fetch).toHaveBeenCalledWith('/api/v1/delivery/update', expect.any(Object));
        });
    });
});