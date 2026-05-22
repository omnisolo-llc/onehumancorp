'use client';

import React from 'react';

export default function Login() {
  return (
    <div>
      <h1>Login</h1>
      <input type="text" placeholder="Email or Username" />
      <input type="password" />
      <button onClick={() => window.location.href = '/dashboard'}>Login</button>
    </div>
  );
}
