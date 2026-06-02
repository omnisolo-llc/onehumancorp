"use client";

import React, { useState } from 'react';

export default function FulfillmentDashboard() {
  const [batches, setBatches] = useState([
    {
      id: 'mock-batch-1',
      subscription_plan_name: 'Monthly Coffee Bean',
      fulfillment_date: '2024-05-05',
      status: 'pending',
      total_boxes: 42,
    }
  ]);

  return (
    <div className="p-4 sm:p-8 max-w-[1440px] mx-auto min-h-screen">
      <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-8 gap-4">
        <h1 className="text-3xl font-bold tracking-tight bg-clip-text text-transparent bg-gradient-to-r from-gray-900 to-gray-600 dark:from-gray-100 dark:to-gray-400">
          Fulfillment Batches
        </h1>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
        <div className="bg-white/80 dark:bg-gray-900/80 backdrop-blur-xl border border-gray-200 dark:border-gray-800 rounded-xl p-6 shadow-sm">
            <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400">
                Active Subscribers
            </h3>
            <div className="text-3xl font-bold text-gray-900 dark:text-white mt-2">
                42
            </div>
        </div>

        <div className="bg-white/80 dark:bg-gray-900/80 backdrop-blur-xl border border-gray-200 dark:border-gray-800 rounded-xl p-6 shadow-sm">
            <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400">
                Upcoming Fulfillment
            </h3>
            <div className="text-3xl font-bold text-gray-900 dark:text-white mt-2">
                42 boxes
            </div>
        </div>
      </div>

      <div className="space-y-4">
        {batches.map((batch) => (
          <div key={batch.id} className="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-xl p-6 shadow-sm transition-all duration-200 hover:shadow-lg hover:shadow-indigo-500/10">
            <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
              <div className="space-y-1">
                <div className="flex items-center gap-3">
                  <h3 className="font-semibold text-lg">{batch.subscription_plan_name}</h3>
                  <span className={`px-2.5 py-0.5 rounded-full text-xs font-medium capitalize ${batch.status === 'pending' ? 'bg-yellow-100 text-yellow-800' : 'bg-gray-100 text-gray-800'}`}>
                    {batch.status}
                  </span>
                </div>
                <p className="text-sm text-gray-500 dark:text-gray-400">
                  Fulfillment Date: <span className="font-medium text-gray-900 dark:text-gray-200">{batch.fulfillment_date}</span>
                </p>
                <p className="text-sm text-gray-500 dark:text-gray-400">
                  Total Boxes: <span className="font-medium text-gray-900 dark:text-gray-200">{batch.total_boxes}</span>
                </p>
              </div>
              <div className="flex gap-2 w-full sm:w-auto">
                <button className="w-full sm:w-auto px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors font-medium">
                   Print Labels ({batch.total_boxes})
                </button>
              </div>
            </div>
          </div>
        ))}
        {batches.length === 0 && (
          <div className="text-center py-12">
            <p className="text-gray-500 dark:text-gray-400">No upcoming fulfillment batches.</p>
          </div>
        )}
      </div>
    </div>
  );
}
