import React from 'react';
export default function Link({ children, href, ...props }: any) {
  return React.createElement('a', { href, ...props }, children);
}
