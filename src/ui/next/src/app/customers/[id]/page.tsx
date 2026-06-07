"use client";

import React, { useEffect, useState } from "react";
import Link from "next/link";
import { useParams } from "next/navigation";
import { TopNav } from "../../components/TopNav";
import { ErrorBoundary } from "../../components/ErrorBoundary";

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

function CustomerDetailContent() {
  const params = useParams();
  const id = params.id as string;
  const [customer, setCustomer] = useState<Customer360 | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchCustomer() {
      if (!id) return;
      try {
        const token = typeof window !== 'undefined' ? localStorage.getItem('token') || '' : '';
        const res = await fetch(`/api/customers/${id}`, {
          headers: {
            'Authorization': token ? `Bearer ${token}` : ''
          }
        });
        if (res.ok) {
          const data = await res.json();
          setCustomer(data);
        }
      } catch (err) {
        console.error("Failed to load customer", err);
      } finally {
        setLoading(false);
      }
    }
    fetchCustomer();
  }, [id]);

  if (loading) {
    return <div className="p-12 text-center text-gray-500">Loading profile...</div>;
  }

  if (!customer) {
    return (
      <div className="p-12 text-center">
        <h2 className="text-2xl font-bold mb-4">Customer not found</h2>
        <Link href="/customers" className="text-blue-600 hover:underline">
          &larr; Back to Customers
        </Link>
      </div>
    );
  }

  return (
    <div className="w-full max-w-3xl mx-auto py-8">
      <div className="mb-6 px-4">
        <Link href="/customers" className="text-blue-600 hover:underline text-sm font-medium mb-6 inline-block">
          &larr; Back to Customers
        </Link>
      </div>

      <div className="px-4">
        {/* Profile Header Card */}
        <div className="glassmorphism p-8 rounded-[32px] mb-8 relative overflow-hidden">
          <div className="absolute top-0 right-0 w-64 h-64 bg-gradient-to-bl from-blue-400/20 to-purple-400/20 rounded-full blur-3xl -mr-20 -mt-20"></div>

          <div className="flex items-center gap-6 relative z-10">
            <div className="w-24 h-24 rounded-full bg-gradient-to-br from-indigo-500 to-purple-600 flex items-center justify-center text-white font-bold text-4xl shadow-md border-4 border-white dark:border-gray-800">
              {customer.customer_id.substring(0, 2).toUpperCase()}
            </div>
            <div>
              <h1 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-1">
                {customer.customer_id}
              </h1>
              <div className="flex items-center gap-3 text-sm text-gray-500 dark:text-gray-400">
                <span>Customer ID: {customer.id.split('-')[0]}</span>
                {customer.created_at && (
                  <>
                    <span className="w-1 h-1 bg-gray-300 rounded-full"></span>
                    <span>Joined {new Date(customer.created_at).toLocaleDateString()}</span>
                  </>
                )}
              </div>
            </div>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {/* Known Preferences Card */}
          <div className="glassmorphism p-6 rounded-[24px]">
            <h3 className="text-lg font-bold text-[#1D1D1F] dark:text-[#F5F5F7] mb-4 flex items-center gap-2">
              <svg className="w-5 h-5 text-indigo-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z" /></svg>
              Known Preferences
            </h3>

            {customer.preferences && Object.keys(customer.preferences).length > 0 ? (
              <div className="space-y-3" data-testid="customer-preferences">
                {Object.entries(customer.preferences).map(([key, value]) => (
                  <div key={key} className="flex flex-col bg-white/50 dark:bg-black/20 p-3 rounded-[12px] border border-gray-100 dark:border-gray-800">
                    <span className="text-xs text-gray-500 uppercase tracking-wider font-medium mb-1">
                      {key.replace(/_/g, ' ')}
                    </span>
                    <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
                      {value as string}
                    </span>
                  </div>
                ))}
              </div>
            ) : (
              <div className="text-sm text-gray-500 bg-gray-50 dark:bg-gray-800/50 p-4 rounded-[12px] text-center">
                No preferences discovered yet.
              </div>
            )}
          </div>

          {/* Contact Details Card */}
          <div className="glassmorphism p-6 rounded-[24px]">
            <h3 className="text-lg font-bold text-[#1D1D1F] dark:text-[#F5F5F7] mb-4 flex items-center gap-2">
              <svg className="w-5 h-5 text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" /></svg>
              Contact Details
            </h3>

            <div className="space-y-4">
              <div className="flex items-center justify-between border-b border-gray-100 dark:border-gray-800 pb-3">
                <span className="text-sm text-gray-500">Email</span>
                <span className="text-sm font-medium">{customer.email || '—'}</span>
              </div>
              <div className="flex items-center justify-between border-b border-gray-100 dark:border-gray-800 pb-3">
                <span className="text-sm text-gray-500">Phone</span>
                <span className="text-sm font-medium">{customer.phone || '—'}</span>
              </div>
              <div className="flex items-center justify-between pb-3">
                <span className="text-sm text-gray-500">Source</span>
                <span className="text-sm font-medium capitalize">Instagram DM</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default function CustomerProfilePage() {
  return (
    <main className="min-h-screen bg-[#F5F5F7] dark:bg-[#000000] pb-24 lg:pb-0">
      <TopNav />
      <div className="lg:pl-[240px] pt-16">
        <ErrorBoundary>
          <CustomerDetailContent />
        </ErrorBoundary>
      </div>
    </main>
  );
}
