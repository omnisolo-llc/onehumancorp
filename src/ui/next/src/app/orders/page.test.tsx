import { render, screen, act } from "@testing-library/react";
import { expect, test, vi } from "vitest";
import { TooltipProvider } from "../../components/TooltipRegistry";
import OrdersPage from "./page";

vi.mock("next/navigation", () => ({
  usePathname: () => "/orders",
  useRouter: () => ({
    push: vi.fn(),
    replace: vi.fn(),
    prefetch: vi.fn(),
    back: vi.fn(),
    forward: vi.fn(),
    refresh: vi.fn(),
  }),
}));

test("does not expose backend API route names in the orders UI", async () => {
  global.fetch = vi.fn(() => Promise.resolve(Response.json([], { status: 200 })));

  await act(async () => {
    render(
      <TooltipProvider>
        <OrdersPage />
      </TooltipProvider>
    );
  });

  expect(screen.getByText("Order List")).toBeDefined();
  expect(screen.queryByText(/\/api\/ui\/orders/)).toBeNull();
});

test('each recorded order has one valid navigation target rather than nested interactive elements', async () => {
  global.fetch = vi.fn(() => Promise.resolve(Response.json([{ id: 'recorded-order', customer_name: 'Recorded customer', total_amount: 15, status: 'paid' }])));
  render(<TooltipProvider><OrdersPage /></TooltipProvider>);
  await screen.findByText('Recorded customer');
  const link = screen.getByRole('link', { name: /View/ });
  expect(link).toHaveAttribute('href', '/orders/recorded-order');
  expect(link.querySelector('button, a, input')).toBeNull();
});
