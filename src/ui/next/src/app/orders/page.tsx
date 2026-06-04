"use client";

import { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function OrdersPage() {
  const router = useRouter();

  // Mock orders
  const [orders] = useState([
    {
      id: 'ORD-7829',
      customerName: 'Alice Johnson',
      items: '2x Vegan Chocolate Cake',
      total: '$45.00',
      status: 'unfulfilled',
      date: 'Oct 12, 2023',
    },
    {
      id: 'ORD-7830',
      customerName: 'Bob Smith',
      items: '1x Custom Birthday Cake',
      total: '$85.00',
      status: 'shipped',
      date: 'Oct 11, 2023',
    }
  ]);

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit text-gray-900">Orders</h1>
        <button onClick={() => router.push('/dashboard')} className="text-sm font-medium text-blue-600 hover:text-blue-800">
          Back to Dashboard
        </button>
      </header>

      <main className="p-6 max-w-4xl mx-auto w-full flex flex-col gap-6">
        <div className="bg-white rounded-2xl shadow-sm border border-gray-100 overflow-hidden">
          <table className="w-full text-left border-collapse">
            <thead>
              <tr className="bg-gray-50 border-b border-gray-100 text-sm text-gray-500 uppercase tracking-wide font-semibold">
                <th className="p-4">Order ID</th>
                <th className="p-4">Date</th>
                <th className="p-4">Customer</th>
                <th className="p-4">Items</th>
                <th className="p-4">Total</th>
                <th className="p-4">Status</th>
                <th className="p-4 text-right">Action</th>
              </tr>
            </thead>
            <tbody>
              {orders.map(order => (
                <tr key={order.id} className="border-b border-gray-50 hover:bg-gray-50 transition-colors">
                  <td className="p-4 font-medium text-gray-900">{order.id}</td>
                  <td className="p-4 text-sm text-gray-600">{order.date}</td>
                  <td className="p-4 text-sm text-gray-900">{order.customerName}</td>
                  <td className="p-4 text-sm text-gray-600 truncate max-w-[200px]">{order.items}</td>
                  <td className="p-4 text-sm font-medium text-gray-900">{order.total}</td>
                  <td className="p-4">
                    <span className={`px-2 py-1 text-xs rounded-full font-medium ${
                      order.status === 'unfulfilled' ? 'bg-yellow-100 text-yellow-800' : 'bg-green-100 text-green-800'
                    }`}>
                      {order.status === 'unfulfilled' ? 'Unfulfilled' : 'Shipped'}
                    </span>
                  </td>
                  <td className="p-4 text-right">
                    <button
                      onClick={() => router.push(`/orders/${order.id}`)}
                      className="px-3 py-1 bg-blue-50 text-blue-600 hover:bg-blue-100 rounded-lg text-sm font-medium transition-colors"
                    >
                      View
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </main>
    </div>
  );
}
