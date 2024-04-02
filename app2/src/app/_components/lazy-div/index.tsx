import React from "react";

const LazyDiv = React.lazy(() =>
  new Promise((resolve) => setTimeout(resolve, 10000)).then(
    () => import("./div")
  )
);

export default LazyDiv;
