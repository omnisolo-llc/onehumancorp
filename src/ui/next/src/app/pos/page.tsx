"use client";

import React, { useState, useEffect } from "react";
import { PlusCircle, ShoppingCart, Wifi, WifiOff } from "lucide-react";

interface Product {
  id: string;
  name: string;
  price: number;
}

interface CartItem extends Product {
  quantity: number;
}

export default function POSPage() {
  const [products, setProducts] = useState<Product[]>([]);
  const [cart, setCart] = useState<CartItem[]>([]);
  const [isOnline, setIsOnline] = useState<boolean>(true);
  const [syncStatus, setSyncStatus] = useState<"idle" | "syncing" | "success" | "error">("idle");
  const [message, setMessage] = useState<string>("");

  useEffect(() => {
    // In a real app, this would fetch from an API or IndexedDB.
    setProducts([
      { id: "1", name: "Custom Cake", price: 45.00 },
      { id: "2", name: "Consultation Hour", price: 100.00 },
      { id: "3", name: "Repair Kit", price: 25.00 },
    ]);

    const handleOnline = () => setIsOnline(true);
    const handleOffline = () => setIsOnline(false);

    window.addEventListener("online", handleOnline);
    window.addEventListener("offline", handleOffline);

    setIsOnline(navigator.onLine);

    return () => {
      window.removeEventListener("online", handleOnline);
      window.removeEventListener("offline", handleOffline);
    };
  }, []);

  const addToCart = (product: Product) => {
    setCart((prev) => {
      const existing = prev.find((item) => item.id === product.id);
      if (existing) {
        return prev.map((item) =>
          item.id === product.id ? { ...item, quantity: item.quantity + 1 } : item
        );
      }
      return [...prev, { ...product, quantity: 1 }];
    });
  };

  const processPayment = async () => {
    if (cart.length === 0) return;

    setSyncStatus("syncing");
    setMessage("Processing payment...");

    try {
      const payload = {
        items: cart,
        total: cart.reduce((sum, item) => sum + item.price * item.quantity, 0),
        offline_id: crypto.randomUUID(), // Unique ID for idempotency and offline sync
        timestamp: new Date().toISOString(),
      };

      if (!isOnline) {
         // Save to offline queue (IndexedDB/localStorage in reality)
         const queue = JSON.parse(localStorage.getItem("pos_offline_queue") || "[]");
         queue.push(payload);
         localStorage.setItem("pos_offline_queue", JSON.stringify(queue));

         setSyncStatus("success");
         setMessage("Saved Offline. Will sync when connection is restored.");
      } else {
         // Process via online API
         const response = await fetch("/api/terminal/charge", {
             method: "POST",
             headers: { "Content-Type": "application/json" },
             body: JSON.stringify(payload),
         });

         if (!response.ok) {
             throw new Error("Payment failed.");
         }

         setSyncStatus("success");
         setMessage("Payment processed successfully!");
      }

      // Clear cart
      setCart([]);

    } catch (error) {
       console.error(error);
       setSyncStatus("error");
       setMessage("An error occurred. Transaction saved for retry.");

       // Fallback to offline queue on error
       const queue = JSON.parse(localStorage.getItem("pos_offline_queue") || "[]");
       queue.push({
           items: cart,
           total: cart.reduce((sum, item) => sum + item.price * item.quantity, 0),
           offline_id: crypto.randomUUID(),
           timestamp: new Date().toISOString()
       });
       localStorage.setItem("pos_offline_queue", JSON.stringify(queue));
       setCart([]);
    }
  };

  const syncOfflineTransactions = async () => {
      if (!isOnline) return;

      const queue = JSON.parse(localStorage.getItem("pos_offline_queue") || "[]");
      if (queue.length === 0) return;

      setSyncStatus("syncing");
      setMessage(`Syncing ${queue.length} offline transactions...`);

      try {
          // In a real app, send batch or individual requests
          for (const tx of queue) {
              await fetch("/api/terminal/charge", {
                  method: "POST",
                  headers: { "Content-Type": "application/json" },
                  body: JSON.stringify(tx)
              });
          }
          localStorage.removeItem("pos_offline_queue");
          setSyncStatus("success");
          setMessage("Offline transactions synced successfully!");
      } catch(error) {
          console.error("Sync failed", error);
          setSyncStatus("error");
          setMessage("Sync failed. Will retry later.");
      }
  }

  useEffect(() => {
      if(isOnline) {
          syncOfflineTransactions();
      }
  }, [isOnline]);


  const total = cart.reduce((sum, item) => sum + item.price * item.quantity, 0);

  return (
    <div className="flex flex-col min-h-screen bg-gray-50 pb-20 p-4 max-w-md mx-auto relative shadow-xl min-[375px]:max-w-[375px]">

      <header className="flex items-center justify-between py-4 mb-4 border-b">
        <h1 className="text-xl font-bold text-gray-800">POS Terminal</h1>
        <div className="flex items-center space-x-2">
           {isOnline ? (
               <span className="flex items-center text-green-600 text-sm font-medium"><Wifi className="w-4 h-4 mr-1"/> Online</span>
           ) : (
               <span className="flex items-center text-amber-600 text-sm font-medium"><WifiOff className="w-4 h-4 mr-1"/> Offline Mode</span>
           )}
        </div>
      </header>

      {/* Product List */}
      <div className="flex-1 overflow-y-auto mb-4 space-y-3">
        <h2 className="text-sm font-semibold text-gray-500 uppercase">Catalog</h2>
        {products.map((p) => (
          <button
            key={p.id}
            onClick={() => addToCart(p)}
            className="w-full flex items-center justify-between p-4 bg-white rounded-xl shadow-sm border border-gray-100 active:scale-[0.98] transition-transform"
          >
            <span className="font-medium text-gray-800">{p.name}</span>
            <div className="flex items-center">
                <span className="text-gray-600 mr-3">${p.price.toFixed(2)}</span>
                <PlusCircle className="text-blue-600 w-5 h-5" />
            </div>
          </button>
        ))}
      </div>

      {/* Cart Summary */}
      <div className="bg-white rounded-xl shadow-lg border border-gray-100 overflow-hidden sticky bottom-0">
        <div className="p-4 border-b bg-gray-50 flex items-center justify-between">
             <div className="flex items-center text-gray-700 font-medium">
                 <ShoppingCart className="w-5 h-5 mr-2" />
                 Cart ({cart.reduce((acc, c) => acc + c.quantity, 0)})
             </div>
             <span className="text-xl font-bold text-gray-900">${total.toFixed(2)}</span>
        </div>

        {cart.length > 0 && (
           <div className="max-h-32 overflow-y-auto p-4 space-y-2">
               {cart.map(c => (
                   <div key={c.id} className="flex justify-between text-sm text-gray-600">
                       <span>{c.quantity}x {c.name}</span>
                       <span>${(c.price * c.quantity).toFixed(2)}</span>
                   </div>
               ))}
           </div>
        )}

        <div className="p-4">
            <button
            onClick={processPayment}
            disabled={cart.length === 0 || syncStatus === "syncing"}
            className="w-full py-3.5 rounded-lg font-semibold text-white bg-blue-600 hover:bg-blue-700 active:bg-blue-800 disabled:opacity-50 transition-colors flex justify-center items-center"
            >
             {syncStatus === "syncing" ? "Processing..." : "Charge via Tap-to-Pay"}
            </button>
        </div>

        {message && (
            <div className={`text-center py-2 text-sm ${syncStatus === 'error' ? 'text-red-600 bg-red-50' : 'text-green-600 bg-green-50'}`}>
                {message}
            </div>
        )}
      </div>
    </div>
  );
}
