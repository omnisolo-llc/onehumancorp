import { render, screen, act } from "@testing-library/react";
import { expect, test, vi } from "vitest";
import { TooltipProvider } from "../../components/TooltipRegistry";
import InventoryDashboard from "./page";

vi.mock("next/navigation", () => ({
  usePathname: () => "/inventory",
  useRouter: () => ({
    push: vi.fn(),
    replace: vi.fn(),
    prefetch: vi.fn(),
    back: vi.fn(),
    forward: vi.fn(),
    refresh: vi.fn(),
  }),
}));

test("does not expose backend API route names in the inventory UI", async () => {
  global.fetch = vi.fn(() => Promise.resolve({
    ok: true,
    json: () => Promise.resolve({ inventory: [] }),
  })) as any;

  await act(async () => {
    render(
      <TooltipProvider>
        <InventoryDashboard />
      </TooltipProvider>
    );
  });

  expect(screen.getByText("Products & Variants")).toBeDefined();
  expect(screen.queryByText(/\/api\/ui\/inventory/)).toBeNull();
});
