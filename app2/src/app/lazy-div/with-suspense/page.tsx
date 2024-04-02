import LazyDiv from "@/app/_components/lazy-div";
import { Suspense } from "react";


export default function Page() {
  return (
    <div>
      <Suspense fallback={<div>It is React.Suspense</div>}>
        <LazyDiv>I&apos;m too lazy to div</LazyDiv>
      </Suspense>
    </div>
  );
}