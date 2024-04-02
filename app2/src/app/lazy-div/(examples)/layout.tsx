export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <>
      Example of LazyDiv usage
      {children}
    </>
  );
}
