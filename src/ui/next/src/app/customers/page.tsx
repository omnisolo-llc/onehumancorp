"use client";

import React, { useEffect, useState } from "react";
import Link from "next/link";
import { TopNav } from "../components/TopNav";
import { ErrorBoundary } from "../components/ErrorBoundary";

interface Customer360 {
  id: string;
  tenant_id: string;
  customer_id: string;
  email?: string;
  phone?: string;
  mood?: string;
  preferences?: Record<string, string>;
  created_at?: string;
  updated_at?: string;
}

function CustomersContent() {
  const [customers, setCustomers] = useState<Customer360[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchCustomers() {
      try {
        const token = typeof window !== 'undefined' ? localStorage.getItem('token') || '' : '';
        const res = await fetch("/api/customers", {
          headers: {
            'Authorization': token ? `Bearer ${token}` : ''
          }
        });
        if (res.ok) {
          const data = await res.json();
          setCustomers(data || []);
        }
      } catch (err) {
        console.error("Failed to load customers", err);
      } finally {
        setLoading(false);
      }
    }
    fetchCustomers();
  }, []);

  return (
    <div className="w-full max-w-4xl mx-auto py-8">
      <div className="mb-8 px-4 flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">Customers</h1>
          <p className="text-sm text-gray-500 dark:text-gray-400 mt-1">Unified view of your customer relationships</p>
        </div>
      </div>

      <div className="px-4">
        {loading ? (
          <div className="p-8 text-center text-gray-500">Loading customers...</div>
        ) : customers.length === 0 ? (
          <div className="glassmorphism p-12 text-center rounded-[24px]">
            <div className="w-16 h-16 bg-blue-100 dark:bg-blue-900/30 text-blue-600 rounded-full flex items-center justify-center mx-auto mb-4">
              <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" /></svg>
            </div>
            <h3 className="text-lg font-semibold mb-2">No customers yet</h3>
            <p className="text-gray-500 text-sm">Customers will automatically appear here as they interact with your business.</p>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {customers.map((c) => (
              <Link href={`/customers/${c.customer_id}`} key={c.id}>
                <div className="glassmorphism p-6 rounded-[24px] hover:border-blue-500/30 transition-all cursor-pointer group">
                  <div className="flex items-center gap-4 mb-4">
                    <div className="w-12 h-12 rounded-full bg-gradient-to-br from-indigo-500 to-purple-600 flex items-center justify-center text-white font-bold text-lg shadow-sm">
                      {c.customer_id.substring(0, 2).toUpperCase()}
                    </div>
                    <div>
                      <h3 className="font-semibold text-gray-900 dark:text-white group-hover:text-blue-600 transition-colors">
                        {c.customer_id}
                      </h3>
                      {c.email && <p className="text-xs text-gray-500">{c.email}</p>}
                    </div>
                  </div>

                  {c.preferences && Object.keys(c.preferences).length > 0 && (
                    <div className="mt-4 pt-4 border-t border-gray-100 dark:border-gray-800">
                      <p className="text-xs text-gray-400 mb-2 font-medium uppercase tracking-wider">Preferences</p>
                      <div className="flex flex-wrap gap-2">
                        {Object.entries(c.preferences).slice(0, 3).map(([k, v]) => (
                          <span key={k} className="inline-block bg-blue-50 dark:bg-blue-900/20 text-blue-700 dark:text-blue-300 px-2 py-1 rounded-[6px] text-xs font-medium border border-blue-100 dark:border-blue-800/50">
                            {v as string}
                          </span>
                        ))}
                        {Object.keys(c.preferences).length > 3 && (
                          <span className="inline-block bg-gray-100 dark:bg-gray-800 text-gray-600 dark:text-gray-400 px-2 py-1 rounded-[6px] text-xs font-medium">
                            +{Object.keys(c.preferences).length - 3} more
                          </span>
                        )}
                      </div>
                    </div>
                  )}
                </div>
              </Link>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

export default function CustomersPage() {
  return (
    <main className="min-h-screen bg-[#F5F5F7] dark:bg-[#000000] pb-24 lg:pb-0">
      <TopNav />
      <div className="lg:pl-[240px] pt-16">
        <ErrorBoundary>
          <CustomersContent />
        </ErrorBoundary>
      </div>
    </main>
  );
}
