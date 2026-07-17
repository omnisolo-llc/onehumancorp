import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import ActorModelPage from "./page";
import { vi, describe, it, expect } from "vitest";
import React from 'react';

global.fetch = vi.fn();

describe("ActorModelPage", () => {
  it("renders the page correctly", () => {
    render(<ActorModelPage />);
    expect(screen.getByText("Actor-Model Message Passing")).toBeTruthy();
  });

  it("handles successful execution", async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ result: "Success result" })
    });

    render(<ActorModelPage />);
    const input = screen.getByLabelText("Message to the Swarm");
    fireEvent.change(input, { target: { value: "Hello swarm" } });

    const button = screen.getByText("Send Message to Swarm");
    fireEvent.click(button);

    await waitFor(() => {
      expect(screen.getByTestId("success-message")).toHaveTextContent("Success result");
    });
  });

  it("handles execution failure", async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: false,
      json: async () => ({ error: "Failed to execute" })
    });

    render(<ActorModelPage />);
    const input = screen.getByLabelText("Message to the Swarm");
    fireEvent.change(input, { target: { value: "Fail swarm" } });

    const button = screen.getByText("Send Message to Swarm");
    fireEvent.click(button);

    await waitFor(() => {
      expect(screen.getByTestId("error-message")).toHaveTextContent("Failed to execute");
    });
  });
});
