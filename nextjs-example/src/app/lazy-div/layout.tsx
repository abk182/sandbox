export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <>
      <h1>Example of LazyDiv usage</h1>
      {children}
    </>
  );
}
