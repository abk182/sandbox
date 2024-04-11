import LazyDiv from "@/app/_components/lazy-div";

export default function Page({ params }: { params: { text: string } }) {
  return (
    <LazyDiv.ReactLazy>I&apos;m too lazy to {params.text}</LazyDiv.ReactLazy>
  );
}
