import LazyDiv from "@/app/_components/lazy-div";
import { Suspense } from "react";

export default function Page() {
  return (
    <div>
      <Suspense fallback={"loading..."}>
        <LazyDiv>I&apos;m too lazy to div</LazyDiv>
      </Suspense>
    </div>
  );
}
