"use client";

import { useState, useEffect } from "react";
import { AppShell } from "../components/AppShell";
import { useCurrency } from "../../lib/localizationStore";

export default function ProductsPage() {
  const { currency } = useCurrency();
  const [products, setProducts] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchProducts = async () => {
      try {
        const res = await fetch(`/api/v1/catalog/products?currency=${currency}`);
        if (res.ok) {
          const data = await res.json();
          setProducts(data);
        }
      } catch (e) {
        console.error("Failed to fetch products", e);
      } finally {
        setLoading(false);
      }
    };
    fetchProducts();
  }, [currency]);

  const formatPrice = (cents: number, c: string) => {
    return `${c === 'USD' ? '$' : c === 'EUR' ? '€' : c === 'GBP' ? '£' : c + ' '}${(cents / 100).toFixed(2)}`;
  };

  return (
    <AppShell
      title="Products"
      subtitle="Review imported catalog items before publishing them to your storefront."
      statusItems={[
        { label: "Catalog", value: String(products.length), tone: "neutral" },
        { label: "Source", value: "API", tone: "good" },
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
          {loading ? (
            <div className="p-4 text-center text-gray-500">Loading catalog...</div>
          ) : products.length === 0 ? (
            <div className="p-8 text-center text-gray-500">No products found. Add a product to get started.</div>
          ) : (
            products.map((product) => (
              <div key={product.title || product.id} className="app-list-item">
                <div>
                  <div className="app-list-title">{product.title || product.name || 'Unnamed Product'}</div>
                  <div className="app-list-subtitle">{formatPrice(product.price_cents, product.currency || currency)}</div>
                </div>
                <span className="app-badge good">{product.status || 'Active'}</span>
              </div>
            ))
          )}
        </div>
      </section>
    </AppShell>
  );
}
