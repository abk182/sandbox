import LazyDiv from "@/app/_components/lazy-div";
import { Suspense } from "react";

export default function Page({ params }: { params: { text: string } }) {
  return (
    <Suspense fallback={"It is React.Suspense ..."}>
      <LazyDiv.ReactLazy>I&apos;m too lazy to {params.text}</LazyDiv.ReactLazy>
    </Suspense>
  );
}
