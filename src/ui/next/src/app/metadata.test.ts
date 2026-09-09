import { describe, expect, test } from "vitest";
import { omniSoloMetadata } from "./metadata";

describe("OmniSolo document metadata", () => {
  test("uses the product name for the default and page titles", () => {
    expect(omniSoloMetadata.title).toEqual({
      default: "OmniSolo",
      template: "%s | OmniSolo OneHumanCorp",
    });
    expect(omniSoloMetadata.description).toBe("OmniSolo business workspace");
  });
});
