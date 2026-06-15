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
  global.fetch = vi.fn(() => Promise.resolve({
    ok: true,
    json: () => Promise.resolve([]),
  })) as any;

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
