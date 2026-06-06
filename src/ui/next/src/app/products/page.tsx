"use client";

import { AppShell } from "../components/AppShell";

const importedProducts = [
  { name: "Chocolate Cake", price: "$20.00", status: "Imported" },
  { name: "Vanilla Celebration Cake", price: "$24.00", status: "Imported" },
  { name: "Wedding Cake Consultation", price: "$75.00", status: "Imported" },
];

export default function ProductsPage() {
  return (
    <AppShell
      title="Products"
      subtitle="Review imported catalog items before publishing them to your storefront."
      statusItems={[
        { label: "Catalog", value: String(importedProducts.length), tone: "good" },
        { label: "Source", value: "Imported", tone: "good" },
      ]}
      actions={[{ label: "New Product", href: "/products/new", primary: true }]}
    >
      <section className="app-panel">
        <div className="app-panel-header">
          <div>
            <div className="app-panel-title">Imported Products</div>
            <div className="app-list-subtitle">Catalog rows staged from the migration workflow.</div>
          </div>
        </div>
        <div className="app-list">
          {importedProducts.map((product) => (
            <div key={product.name} className="app-list-item">
              <div>
                <div className="app-list-title">{product.name}</div>
                <div className="app-list-subtitle">{product.price}</div>
              </div>
              <span className="app-badge good">{product.status}</span>
            </div>
          ))}
        </div>
      </section>
    </AppShell>
  );
}
