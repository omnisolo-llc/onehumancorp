import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import DeerFlowOrchestrationPage from "./page";
import { vi, describe, it, expect } from "vitest";

global.fetch = vi.fn();

describe("DeerFlowOrchestrationPage", () => {
  it("renders correctly", () => {
    render(<DeerFlowOrchestrationPage />);
    expect(screen.getByText("DeerFlow Sub-agent Orchestration")).toBeInTheDocument();
  });

  it("handles execution", async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ result: "Final Synthesis" }),
    });

    render(<DeerFlowOrchestrationPage />);

    // Set text to enable the button
    const textareas = screen.getAllByRole("textbox");
    fireEvent.change(textareas[0], {
      target: { value: 'Analyze market' }
    });

    // Click button
    const buttons = screen.getAllByRole("button");
    const runBtn = buttons.find(b => b.textContent?.includes("Execute Task via DeerFlow"));
    if (runBtn) {
        fireEvent.click(runBtn);
    }

    await waitFor(() => {
      expect(screen.getByText(/Final Synthesis/i)).toBeInTheDocument();
    });
  });

  it("handles error", async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: false,
      json: async () => ({ error: "Orchestration Failed" }),
    });

    render(<DeerFlowOrchestrationPage />);

    // Set text to enable the button
    const textareas = screen.getAllByRole("textbox");
    fireEvent.change(textareas[0], {
      target: { value: 'Analyze market' }
    });

    // Click button
    const buttons = screen.getAllByRole("button");
    const runBtn = buttons.find(b => b.textContent?.includes("Execute Task via DeerFlow"));

    if (runBtn) {
        fireEvent.click(runBtn);
    }

    await waitFor(() => {
      expect(screen.getByText(/Orchestration Failed/i)).toBeInTheDocument();
    });
  });
});
