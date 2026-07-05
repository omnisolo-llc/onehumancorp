"use client";

import React, { useState, useEffect } from "react";
import StripeTerminalClient from "../terminal/StripeTerminalClient";

export default function OmnichannelCartPage() {
  const [tenant, setTenant] = useState<string>("test_tenant");
  const [cartId, setCartId] = useState<string | null>(null);
  const [cartItems, setCartItems] = useState<any[]>([]);
  const [cartTotal, setCartTotal] = useState<number>(0);
  const [status, setStatus] = useState<string>("");
  const [productId, setProductId] = useState<string>("prod_terminal_123");
  const [isProcessing, setIsProcessing] = useState<boolean>(false);

  useEffect(() => {
    if (typeof window !== "undefined") {
      const storedTenant = localStorage.getItem("tenant");
      if (storedTenant) setTenant(storedTenant);
    }
  }, []);

  const createCart = async () => {
    setStatus("Creating cart...");
    setIsProcessing(true);
    try {
      const res = await fetch("/api/v1/cart", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          channel: "in_store",
          currency: "usd",
        }),
      });
      const data = await res.json();
      if (res.ok) {
        setCartId(data.id);
        setStatus("Cart created: " + data.id);
      } else {
        setStatus("Failed to create cart: " + data.error);
      }
    } catch (e: any) {
      setStatus("Error: " + e.message);
    }
    setIsProcessing(false);
  };

  const addItemToCart = async () => {
    if (!cartId) return;
    setStatus("Adding item to cart...");
    setIsProcessing(true);
    try {
      const res = await fetch(`/api/v1/cart/${cartId}/items`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          product_id: productId,
          quantity: 1,
          unit_price_cents: 1500, // $15.00
        }),
      });
      const data = await res.json();
      if (res.ok) {
        setCartItems([...cartItems, { product_id: productId, quantity: 1, unit_price_cents: 1500 }]);
        setCartTotal(cartTotal + 1500);
        setStatus("Item added successfully.");
      } else {
        setStatus("Failed to add item: " + data.error);
      }
    } catch (e: any) {
      setStatus("Error: " + e.message);
    }
    setIsProcessing(false);
  };

  return (
    <div className="flex flex-col min-h-screen bg-[#F8F9FA] text-gray-900 p-4 md:p-8 font-inter">
      <div className="max-w-md mx-auto w-full bg-white backdrop-blur-[30px] saturate-[210%] rounded-[24px] shadow-lg p-6 border border-white/40 mt-10">
        <h1 className="text-2xl font-bold font-outfit mb-4 text-gray-900 tracking-tight">New In-Store Sale</h1>
        <p className="text-sm text-gray-600 mb-6 font-medium bg-gray-50 p-2 rounded-lg" id="status-message">Status: {status}</p>

        {!cartId ? (
          <button
            id="create-cart-btn"
            onClick={createCart}
            disabled={isProcessing}
            className="w-full bg-[#0066FF] text-white py-3 min-h-[44px] rounded-xl font-bold shadow-md shadow-blue-500/20 hover:bg-blue-700 disabled:opacity-50 transition-all active:scale-[0.98]"
          >
            Create Omnichannel Cart
          </button>
        ) : (
          <div className="space-y-4">
            <div className="bg-gray-50 p-4 rounded-xl border border-gray-100">
              <h2 className="text-xs font-bold text-gray-500 uppercase tracking-wider mb-2">Cart ID: {cartId.substring(0, 8)}...</h2>
              <div className="flex gap-2">
                <input
                  type="text"
                  value={productId}
                  onChange={(e) => setProductId(e.target.value)}
                  placeholder="Product ID"
                  className="flex-1 border border-gray-200 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-[#0066FF]/50"
                  id="product-input"
                />
                <button
                  id="add-item-btn"
                  onClick={addItemToCart}
                  disabled={isProcessing}
                  className="bg-gray-900 text-white px-4 py-2 min-h-[44px] rounded-lg text-sm font-bold hover:bg-black disabled:opacity-50 transition-all active:scale-[0.98]"
                >
                  Add Item
                </button>
              </div>
            </div>

            {cartItems.length > 0 && (
              <div className="mt-4">
                <div className="flex justify-between items-center font-bold text-lg border-t border-gray-100 pt-4 mb-2">
                  <span>Total Due</span>
                  <span className="text-2xl font-outfit">${(cartTotal / 100).toFixed(2)}</span>
                </div>

                <div className="mt-6 border-t border-gray-100 pt-4" id="tap-to-pay-container">
                  {/* Stripe Terminal Client for Tap-to-Pay */}
                  <StripeTerminalClient
                    amount={cartTotal}
                    productId={cartItems[0].product_id}
                    tenantId={tenant}
                  />
                </div>
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
