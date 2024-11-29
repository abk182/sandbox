import React from "react";
import dynamic from "next/dynamic";

const ReactLazy = React.lazy(() =>
  new Promise((resolve) => setTimeout(resolve, 10000)).then(
    () => import("../div")
  )
);

const NextDynamic = dynamic(() =>
  new Promise((resolve) => setTimeout(resolve, 10000)).then(
    () => import("../div")
  )
);

const LazyDiv = { ReactLazy, NextDynamic };

export default LazyDiv;
