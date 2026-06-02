import React from 'react';
const Link = ({ children, href, ...rest }: any) => {
  return React.createElement('a', { href, ...rest }, children);
};
export default Link;
