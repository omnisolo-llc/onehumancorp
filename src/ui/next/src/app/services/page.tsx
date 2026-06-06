'use client';
import React, { useState, useEffect } from 'react';
import Link from 'next/link';

export default function ServicesPage() {
  const [services, setServices] = useState<any[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const fetchServices = async () => {
      setIsLoading(true);
      try {
        const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'my-store' : 'my-store';
        const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'anon' : 'anon';
        const res = await fetch('/api/onboarding/state', {
          headers: {
            'X-Tenant-ID': tenantId,
            'X-User-ID': userId,
          }
        });
        if (res.ok) {
          const data = await res.json();
          // Based on NewServicePage, it saves as JSON.stringify({ services: [...] }) or just an object
          if (data && data.services && Array.isArray(data.services)) {
            setServices(data.services);
          } else if (data && data.wizardState && data.wizardState.services) {
            setServices(data.wizardState.services);
          } else if (data && data.wizardState && data.wizardState.firstProductName) {
            setServices([{
               title: data.wizardState.firstProductName,
               description: "Initial product",
               price: data.wizardState.firstProductPrice || '0.00'
            }]);
          } else {
            setServices([]);
          }
        }
      } catch (e) {
        console.error('Failed to fetch services', e);
      } finally {
        setIsLoading(false);
      }
    };
    fetchServices();
  }, []);

  return (
    <div id="services-screen" className="max-w-4xl mx-auto p-6 min-h-screen bg-[#F5F5F7] dark:bg-[#1D1D1F]">
      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-3xl font-bold text-[#1D1D1F] dark:text-[#F5F5F7]">Service Manager</h1>
          <p className="text-gray-500 dark:text-[#A1A1A6] mt-1">Manage your offerings</p>
        </div>
        <Link
          href="/services/new"
          className="bg-[#0066FF] hover:bg-[#0052cc] text-white px-4 py-2 rounded-lg font-semibold shadow-sm transition-all active:scale-[0.98]"
        >
          Add Service
        </Link>
      </div>

      {isLoading ? (
        <div className="flex justify-center items-center h-48">
          <div className="w-8 h-8 border-4 border-[#0066FF]/20 border-t-[#0066FF] rounded-full animate-spin"></div>
        </div>
      ) : services.length === 0 ? (
        <div className="bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 rounded-[16px] p-12 text-center shadow-sm">
          <div className="w-16 h-16 bg-gray-100 dark:bg-gray-800 rounded-full flex items-center justify-center mx-auto mb-4">
            <svg className="w-8 h-8 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
            </svg>
          </div>
          <h3 className="text-xl font-bold text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">No services yet</h3>
          <p className="text-gray-500 dark:text-[#A1A1A6] mb-6">Get started by creating your first service offering.</p>
          <Link
            href="/services/new"
            className="inline-flex bg-[#1D1D1F] dark:bg-white text-white dark:text-[#1D1D1F] px-6 py-3 rounded-lg font-semibold shadow-sm transition-all hover:bg-black dark:hover:bg-gray-200 active:scale-[0.98]"
          >
            Create your first service
          </Link>
        </div>
      ) : (
        <div className="grid gap-4 md:grid-cols-2">
          {services.map((service, i) => (
            <div key={i} className="bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 rounded-[16px] p-6 shadow-sm transition-all hover:shadow-md">
              <div className="flex justify-between items-start mb-2">
                <h3 className="text-lg font-bold text-[#1D1D1F] dark:text-[#F5F5F7]">{service.title || 'Untitled Service'}</h3>
                {service.price && (
                  <span className="bg-[#34C759]/10 text-[#34C759] font-bold px-2 py-1 rounded-[6px] text-sm">
                    ${parseFloat(service.price).toFixed(2)}
                  </span>
                )}
              </div>
              <p className="text-gray-600 dark:text-[#A1A1A6] text-sm mb-4 line-clamp-2">
                {service.description || 'No description provided.'}
              </p>
              <div className="flex gap-2">
                <button className="text-sm font-semibold text-[#0066FF] hover:text-[#0052cc]">Edit</button>
                <button className="text-sm font-semibold text-[#FF3B30] hover:text-[#cc2f26]">Delete</button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
