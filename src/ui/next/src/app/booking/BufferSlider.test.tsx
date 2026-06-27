import { render, screen, fireEvent } from "@testing-library/react";
import { BufferSlider } from "./BufferSlider";
import { expect, test, vi } from "vitest";

test("BufferSlider updates on change", () => {
  const onChange = vi.fn();
  render(<BufferSlider value={30} onChange={onChange} label="Test Buffer" />);

  const slider = screen.getByRole("slider");
  fireEvent.change(slider, { target: { value: "45" } });

  expect(onChange).toHaveBeenCalledWith(45);
});
