"use client";

import { useState, useEffect } from "react";
import Link from "next/link";
import { AppShell } from "../components/AppShell";

export default function EmbedGeneratorPage() {
  const [productName, setProductName] = useState("Custom Cake Deposit");
  const [price, setPrice] = useState("$50.00");
  const [theme, setTheme] = useState("light");
  const [tenantId, setTenantId] = useState("e2e-tenant");
  const [embedCode, setEmbedCode] = useState("");
  const [embedUrl, setEmbedUrl] = useState("");
  const [showPreview, setShowPreview] = useState(false);

  useEffect(() => {
    if (typeof window !== "undefined") {
      const storedTenant = localStorage.getItem("tenant_id") || localStorage.getItem("tenant");
      if (storedTenant) setTenantId(storedTenant);
    }
  }, []);

  const handleGenerate = () => {
    const finalProductName = productName.trim() || "Custom Offer";
    const finalPrice = price.trim() || "$0.00";

    const params = new URLSearchParams({
      tenant: tenantId,
      product_name: finalProductName,
      price: finalPrice,
      theme: theme
    });

    const url = `${window.location.origin}/api/v1/growth/storefront/embed?${params.toString()}`;
    const code = `<iframe src="${url}" width="100%" height="180" style="border:none; border-radius:12px; box-shadow:0 4px 12px rgba(0,0,0,0.1);" title="${finalProductName} - Embed"></iframe>`;

    setEmbedUrl(url);
    setEmbedCode(code);
    setShowPreview(true);
  };

  return (
    <AppShell title="Offer Embed Generator">
      <div className="max-w-2xl mx-auto py-8 px-4 sm:px-6 lg:px-8">
        <div className="glassmorphism rounded-2xl p-8 shadow-xl border border-white/40 dark:border-white/10 relative overflow-hidden">
          <div className="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-blue-500 to-indigo-600"></div>

          <div className="text-center mb-8">
            <h1 className="text-3xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Offer Embed Generator</h1>
            <p className="text-gray-600 dark:text-gray-400">Customize your widget</p>
          </div>

          <div className="space-y-6">
            <div>
              <label htmlFor="product-name" className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Product / Service Name
              </label>
              <input
                type="text"
                id="product-name"
                className="w-full px-4 py-3 rounded-xl border border-gray-200 dark:border-gray-700 bg-white/50 dark:bg-gray-800/50 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
                placeholder="e.g. Custom Cake Deposit"
                value={productName}
                onChange={(e) => setProductName(e.target.value)}
              />
            </div>

            <div>
              <label htmlFor="price" className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Price
              </label>
              <input
                type="text"
                id="price"
                className="w-full px-4 py-3 rounded-xl border border-gray-200 dark:border-gray-700 bg-white/50 dark:bg-gray-800/50 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
                placeholder="e.g. $50.00"
                value={price}
                onChange={(e) => setPrice(e.target.value)}
              />
            </div>

            <div>
              <label htmlFor="theme" className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Theme
              </label>
              <select
                id="theme"
                className="w-full px-4 py-3 rounded-xl border border-gray-200 dark:border-gray-700 bg-white/50 dark:bg-gray-800/50 focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
                value={theme}
                onChange={(e) => setTheme(e.target.value)}
              >
                <option value="light">Light</option>
                <option value="dark">Dark</option>
              </select>
            </div>

            <button
              id="generate-btn"
              onClick={handleGenerate}
              className="w-full bg-blue-600 hover:bg-blue-700 text-white font-bold py-4 rounded-xl shadow-lg transition-all hover:shadow-blue-500/25 active:scale-[0.98]"
            >
              Generate Widget
            </button>
          </div>

          {showPreview && (
            <div className="mt-10 pt-8 border-t border-gray-200 dark:border-gray-700 animate-in fade-in slide-in-from-bottom-4 duration-500">
              <h2 className="text-xl font-bold text-gray-900 dark:text-white mb-6 text-center">Live Preview</h2>

              <div className="w-full max-w-[320px] mx-auto h-[180px] mb-8 bg-gray-50 dark:bg-gray-900 rounded-xl overflow-hidden flex items-center justify-center">
                <iframe
                  src={embedUrl}
                  width="100%"
                  height="180"
                  style={{ border: 'none', borderRadius: '12px', boxShadow: '0 4px 12px rgba(0,0,0,0.1)' }}
                  title={`${productName} - Embed`}
                />
              </div>

              <div className="space-y-2">
                <label htmlFor="embed-code" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                  HTML Embed Code (Copy & Paste)
                </label>
                <textarea
                  id="embed-code"
                  readOnly
                  value={embedCode}
                  className="w-full h-32 px-4 py-3 rounded-xl border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-900 text-sm font-mono focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
                />
              </div>
            </div>
          )}

          <div className="mt-8 text-center">
            <Link href="/dashboard" className="text-blue-600 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300 font-medium inline-flex items-center gap-2">
              <span>&larr;</span> Back to Dashboard
            </Link>
          </div>
        </div>
      </div>
    </AppShell>
  );
}
