import React, { Suspense, useEffect } from "react";

export const rootDomNodeId = "app0.app1";

import "./index.global.css";

const ReactLazy = React.lazy(() => import("./main"));

export const Ui = () => {
  return (
    <div className={"ui"}>
      <Suspense fallback={"loading..."}>
        <ReactLazy />
      </Suspense>
    </div>
  );
};
