import fs from "node:fs";
import path from "node:path";

const dynamicValues: Record<string, string> = {
  articleId: "getting-started-1",
  id: "e2e-route-record",
  tenant: "e2e-tenant",
  customer_id: "e2e-customer",
  workflowId: "e2e-workflow",
};

function routeFromPageFile(root: string, file: string): string {
  const directory = path.relative(root, path.dirname(file));
  const segments = directory
    .split(path.sep)
    .filter((segment) => segment && !segment.startsWith("(") && !segment.startsWith("@"))
    .map((segment) => {
      const parameter = segment.match(/^\[([^.[\]]+)\]$/)?.[1];
      return parameter ? (dynamicValues[parameter] ?? `e2e-${parameter}`) : segment;
    });
  return `/${segments.join("/")}`.replace(/\/$/, "") || "/";
}

export function discoverApplicationRoutes(root = path.resolve(__dirname, "../app")): string[] {
  const pages: string[] = [];
  const walk = (directory: string) => {
    for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
      const fullPath = path.join(directory, entry.name);
      if (entry.isDirectory()) walk(fullPath);
      if (entry.isFile() && entry.name === "page.tsx") pages.push(fullPath);
    }
  };
  walk(root);
  return [...new Set(pages.map((file) => routeFromPageFile(root, file)))].sort();
}
