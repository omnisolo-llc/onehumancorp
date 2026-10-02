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
    ).toBeInTheDocument();
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
    expect(screen.queryByText("undefined")).not.toBeInTheDocument();
  });
});

test('blank and whitespace topics visibly hold generation without a request',()=>{
 fetchMock.mockReset();vi.stubGlobal('fetch',fetchMock);render(<NewProposalPage/>);const button=screen.getByRole('button',{name:'Generate Proposal'});expect(button).toBeDisabled();expect(screen.getByRole('status',{name:'Proposal requirements'})).toHaveTextContent(/project brief/i);
 fireEvent.change(screen.getByLabelText('Project Brief / Topic'),{target:{value:'   '}});expect(button).toBeDisabled();fireEvent.click(button);expect(fetchMock).not.toHaveBeenCalled();
 fireEvent.change(screen.getByLabelText('Project Brief / Topic'),{target:{value:'Actual reviewed topic'}});expect(button).toBeEnabled();
});
