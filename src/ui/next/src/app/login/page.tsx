"use client";

import { useRouter } from "next/navigation";

export default function LoginPage() {
  const router = useRouter();

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50 p-4">
      <div className="bg-white p-8 rounded-2xl shadow-xl w-full max-w-sm">
        <h1 className="text-2xl font-bold mb-6 text-center text-black">Login</h1>
        <div className="space-y-4">
          <input
            type="text"
            placeholder="Email or Username"
            className="w-full p-3 border rounded-lg text-black"
          />
          <input
            type="password"
            placeholder="Password"
            className="w-full p-3 border rounded-lg text-black"
          />
          <button
            className="w-full bg-blue-600 text-white font-bold py-3 rounded-lg"
            onClick={() => router.push('/dashboard')}
          >
            Login
          </button>
        </div>

        <div className="mt-8 pt-6 border-t border-gray-100 text-center">
          <p className="text-sm text-gray-500 mb-4">New here?</p>
          <button
            onClick={() => router.push('/business-setup')}
            className="w-full bg-gray-900 text-white font-bold py-3 rounded-lg"
          >
            Start Business Setup
          </button>
        </div>
      </div>
    </div>
  );
}
