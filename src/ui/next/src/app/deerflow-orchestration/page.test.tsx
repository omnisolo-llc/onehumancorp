import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import DeerFlowOrchestrationPage from "./page";
import { vi, describe, it, expect } from "vitest";
import { observeButtonStates } from "../../../../../../scripts/playwright/button-states.mjs";

global.fetch = vi.fn();

describe("DeerFlowOrchestrationPage", () => {
  it("retains loading evidence after a fast rejected request has settled", async () => {
    const error = "Network transport is unavailable in an offline DOM fixture";
    vi.mocked(global.fetch).mockRejectedValueOnce(new Error(error));
    render(<DeerFlowOrchestrationPage />);
    const textarea = screen.getByRole("textbox");
    fireEvent.change(textarea, { target: { value: "Analyze market" } });
    const button = screen.getByRole<HTMLButtonElement>("button", { name: "Execute Task via DeerFlow" });
    const observation = observeButtonStates(button);
    try {
      fireEvent.click(button);
      await waitFor(() => expect(screen.getByText(error)).toBeInTheDocument());
      expect(observation.states).toContainEqual({ disabled: true, text: "Orchestrating Sub-agents..." });
      expect(button).toBeEnabled();
      expect(button).toHaveTextContent("Execute Task via DeerFlow");
      expect(textarea).toBeEnabled();
      expect(screen.queryByRole("heading", { name: "Synthesized Result" })).not.toBeInTheDocument();
    } finally {
      observation.disconnect();
    }
  });

  it("renders correctly", () => {
    render(<DeerFlowOrchestrationPage />);
    expect(screen.getByText("DeerFlow Sub-agent Orchestration")).toBeInTheDocument();
  });

  it("handles execution", async () => {
    vi.mocked(global.fetch, { partial: true }).mockResolvedValueOnce({
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
    vi.mocked(global.fetch, { partial: true }).mockResolvedValueOnce({
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
