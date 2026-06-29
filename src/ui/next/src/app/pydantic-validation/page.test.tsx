import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import PydanticValidationPage from "./page";
import { vi, describe, it, expect } from "vitest";

global.fetch = vi.fn();

describe("PydanticValidationPage", () => {
  it("renders correctly", () => {
    render(<PydanticValidationPage />);
    expect(screen.getByText("Pydantic-First Tool Schema Validation")).toBeInTheDocument();
  });

  it("handles valid execution", async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        result: "Validation passed successfully",
      }),
    });

    render(<PydanticValidationPage />);

    const selects = screen.getAllByRole("combobox");
    fireEvent.change(selects[0], {
      target: { value: 'TopicRetrieve' }
    });

    const textareas = screen.getAllByRole("textbox");
    fireEvent.change(textareas[0], {
      target: { value: '{"foo":"bar"}' }
    });

    const buttons = screen.getAllByRole("button");
    const runBtn = buttons.find(b => b.textContent?.includes("Validate Tool Payload"));
    if (runBtn) {
        fireEvent.click(runBtn);
    }

    await waitFor(() => {
      expect(screen.getByText(/Validation Successful/i)).toBeInTheDocument();
    });
  });

  it("handles validation error execution", async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: false,
      json: async () => ({ error: "Validation Loop Failed" }),
    });

    render(<PydanticValidationPage />);

    const selects = screen.getAllByRole("combobox");
    fireEvent.change(selects[0], {
      target: { value: 'TopicRetrieve' }
    });

    const textareas = screen.getAllByRole("textbox");
    fireEvent.change(textareas[0], {
      target: { value: '{"foo":"bar"}' }
    });

    const buttons = screen.getAllByRole("button");
    const runBtn = buttons.find(b => b.textContent?.includes("Validate Tool Payload"));
    if (runBtn) {
        fireEvent.click(runBtn);
    }

    await waitFor(() => {
      expect(screen.getByText(/Validation Failed/i)).toBeInTheDocument();
    });
  });
});
