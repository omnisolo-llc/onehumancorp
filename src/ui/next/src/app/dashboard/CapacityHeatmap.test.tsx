import { render, screen } from "@testing-library/react";
import { CapacityHeatmap } from "./CapacityHeatmap";
import { vi, expect, test } from "vitest";
import { TooltipProvider } from "../../components/TooltipRegistry";

vi.stubGlobal("fetch", vi.fn(async () => ({
  ok: true,
  json: async () => ([])
})));

test("CapacityHeatmap renders without crashing", () => {
  render(
    <TooltipProvider>
      <CapacityHeatmap tenant="test-tenant" />
    </TooltipProvider>
  );
  expect(screen.getByText("Workload Capacity")).toBeDefined();
});
