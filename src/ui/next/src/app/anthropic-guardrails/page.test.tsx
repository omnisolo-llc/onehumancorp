import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import AnthropicGuardrailsPage from "./page";
import { vi, describe, it, expect } from "vitest";

global.fetch = vi.fn();

describe("AnthropicGuardrailsPage", () => {
  it("renders correctly", () => {
    render(<AnthropicGuardrailsPage />);
    expect(screen.getByText("Anthropic 3-Stage Tool Gating")).toBeInTheDocument();
  });

  it("handles valid execution", async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ result: "Tool executed successfully" }),
    });

    render(<AnthropicGuardrailsPage />);

    // Set text to enable the button
    const textinputs = screen.getAllByRole("textbox");
    fireEvent.change(textinputs[0], {
      target: { value: 'execute_bash' }
    });

    // Click button
    const buttons = screen.getAllByRole("button");
    const runBtn = buttons.find(b => b.textContent?.includes("Check Tool Guardrails"));
    if (runBtn) {
        fireEvent.click(runBtn);
    }

    await waitFor(() => {
      expect(screen.getByText(/Guardrails Passed/i)).toBeInTheDocument();
    });
  });

  it("handles guardrail error execution", async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: false,
      json: async () => ({ error: "Permission check failed" }),
    });

    render(<AnthropicGuardrailsPage />);

    // Set text to enable the button
    const textinputs = screen.getAllByRole("textbox");
    fireEvent.change(textinputs[0], {
      target: { value: 'rm' }
    });

    // Click button
    const buttons = screen.getAllByRole("button");
    const runBtn = buttons.find(b => b.textContent?.includes("Check Tool Guardrails"));

    if (runBtn) {
        fireEvent.click(runBtn);
    }

    await waitFor(() => {
      expect(screen.getByText(/Guardrail Tripped/i)).toBeInTheDocument();
      expect(screen.getByText(/Permission check failed/i)).toBeInTheDocument();
    });
  });
});
