"use client";

import { useEffect, useState } from "react";
import { AppShell } from "../components/AppShell";

type Product = {
  id: string;
  name: string;
  price: string;
  status: string;
};

export default function ProductsPage() {
  const [products, setProducts] = useState<Product[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchProducts = async () => {
      try {
        const res = await fetch("/api/products");
        if (!res.ok) {
          throw new Error("Failed to fetch products");
        }
        const data = await res.json();
        setProducts(data);
      } catch (err: any) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    };

    fetchProducts();
  }, []);

  return (
    <AppShell
      title="Products"
      subtitle="Manage your catalog items for your storefront."
      statusItems={[
        { label: "Catalog", value: String(products.length), tone: "good" },
        { label: "Source", value: "Database", tone: "good" },
      ]}
      actions={[{ label: "New Product", href: "/products/new", primary: true }]}
    >
      <section className="app-panel">
        <div className="app-panel-header">
          <div>
            <div className="app-panel-title">Your Products</div>
            <div className="app-list-subtitle">Items currently available in your catalog.</div>
          </div>
        </div>
        <div className="app-list">
          {loading && (
            <div className="p-4 text-center text-gray-500">Loading products...</div>
          )}
          {!loading && error && (
            <div className="p-4 text-center text-red-500">Error loading products: {error}</div>
          )}
          {!loading && !error && products.length === 0 && (
            <div className="p-4 text-center text-gray-500">
              No products found. Click "New Product" to add one.
            </div>
          )}
          {!loading && !error && products.map((product) => (
            <div key={product.id} className="app-list-item">
              <div>
                <div className="app-list-title">{product.name}</div>
                <div className="app-list-subtitle">{product.price}</div>
              </div>
              <span className="app-badge good">{product.status || 'Published'}</span>
            </div>
          ))}
        </div>
      </section>
    </AppShell>
  );
}
