import LazyDiv from "@/app/_components/lazy-div";
import { Suspense } from "react";

export default function Page() {
  return (
    <Suspense fallback={"It is React.Suspense ..."}>
      <LazyDiv.ReactLazy>I&apos;m too lazy to div</LazyDiv.ReactLazy>
    </Suspense>
  );
}
