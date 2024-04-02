import { Suspense } from "react";

export default function App({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return children;
}
