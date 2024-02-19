import React from "react";
import { hydrateRoot } from "react-dom/client";
import { Ui, rootDomNodeId } from "../ui";

hydrateRoot(document.getElementById(rootDomNodeId), <Ui />);
