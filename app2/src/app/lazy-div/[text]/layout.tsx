export default function Layout({
  children,
  params,
}: Readonly<{
  children: React.ReactNode;
  params: { text: string };
}>) {
  return <div id={params.text}>{children}</div>;
}
