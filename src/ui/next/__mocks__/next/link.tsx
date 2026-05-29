import React from "react";
export default function Link({ children, href }: any) {
  return <a href={href}>{children}</a>;
}
