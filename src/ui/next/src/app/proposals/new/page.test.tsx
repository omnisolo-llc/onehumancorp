import { fireEvent, render, screen } from "@testing-library/react";
import { beforeEach, describe, expect, test, vi } from "vitest";
import NewProposalPage from "./page";

const fetchMock = vi.fn<typeof fetch>();

describe("NewProposalPage", () => {
  beforeEach(() => {
    fetchMock.mockReset();
    vi.stubGlobal("fetch", fetchMock);
  });

  test("renders the generated narrative proposal", async () => {
    fetchMock.mockResolvedValue(
      Response.json({ proposal: "Bakery website proposal" }),
    );
    render(<NewProposalPage />);

    fireEvent.change(screen.getByLabelText("Project Brief / Topic"), {
      target: { value: "Bakery website" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Generate Proposal" }));

    expect(
      await screen.findByText("Bakery website proposal"),
    ).toBeTruthy();
  });

  test("shows an accessible stable error for a failed draft", async () => {
    fetchMock.mockResolvedValue(
      Response.json({ error: "provider secret" }, { status: 502 }),
    );
    render(<NewProposalPage />);

    fireEvent.change(screen.getByLabelText("Project Brief / Topic"), {
      target: { value: "Bakery website" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Generate Proposal" }));

    expect(await screen.findByRole("alert")).toHaveTextContent(
      "Failed to draft proposal",
    );
    expect(screen.queryByText("undefined")).not.toBeTruthy();
  });
});
