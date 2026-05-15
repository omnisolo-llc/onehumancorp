'use client';
import React, { useState } from 'react';

export default function Login() {
  const [isSignUp, setIsSignUp] = useState(false);

  if (isSignUp) {
    return (
      <div className="flex flex-col items-center justify-center min-h-screen">
        <h1 className="text-3xl mb-4">Sign Up</h1>
        <button className="bg-white border p-2 mb-2 w-64 rounded shadow flex items-center justify-center gap-2">One-tap Google SSO</button>
        <button className="bg-black text-white p-2 mb-4 w-64 rounded shadow flex items-center justify-center gap-2">One-tap Apple SSO</button>
        <div className="my-2 border-b w-64 text-center leading-[0.1em]"><span className="bg-white px-2">OR</span></div>
        <input className="border p-2 mb-2 w-64" type="text" placeholder="Email or Username" />
        <input className="border p-2 mb-4 w-64" type="password" placeholder="Password" />
        <button className="bg-blue-600 text-white px-4 py-2 rounded w-64">Sign Up</button>
        <button className="mt-4 text-blue-500" onClick={() => setIsSignUp(false)}>Already have an account? Login</button>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center justify-center min-h-screen">
      <h1 className="text-3xl mb-4">Login</h1>
      <button className="bg-white border p-2 mb-2 w-64 rounded shadow flex items-center justify-center gap-2">One-tap Google SSO</button>
      <button className="bg-black text-white p-2 mb-4 w-64 rounded shadow flex items-center justify-center gap-2">One-tap Apple SSO</button>
      <div className="my-2 border-b w-64 text-center leading-[0.1em]"><span className="bg-white px-2">OR</span></div>
      <input className="border p-2 mb-2 w-64" type="text" placeholder="Email or Username" />
      <input className="border p-2 mb-4 w-64" type="password" placeholder="Password" />
      <button className="bg-blue-600 text-white px-4 py-2 rounded w-64">Login</button>
      <button className="bg-blue-600 text-white px-4 py-2 rounded w-64">Show</button>
      <button className="mt-4 text-blue-500" onClick={() => setIsSignUp(true)}>Don't have an account? Sign Up</button>
    </div>
  );
}
