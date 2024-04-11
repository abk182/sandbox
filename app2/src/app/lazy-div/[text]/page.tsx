import { ReactNode } from "react";

export default function Layout({
  children,
  params,
}: Readonly<{
  children: ReactNode;
  params: { text: string };
}>) {
  return (
    <>
      <div>{params.text}</div>
      <div>{children}</div>
    </>
  );
}
