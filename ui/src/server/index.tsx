import React from "react";
import { renderToStaticMarkup } from "react-dom/server";
import express from "express";
import path from "path";
import { App, appId } from "../app";

const startServer = () => {
    const assetsFolderName = 'assets'
    const port = 8080;
    const app = express();

    app.use(`/${assetsFolderName}`, express.static(path.join(__dirname, `/${assetsFolderName}`)));

    app.get("/", (req, res) => {
      res.end(
        `<!DOCTYPE html>
          <html>
            <body>
              <div id=${appId}>${renderToStaticMarkup(<App />)}</div>
              <script src="${assetsFolderName}/client.js"></script>
            </body>
          </html>`
      );
    });

    app.listen(port, () => {
      console.log(`App listening on port ${port}`);
    });
};

startServer();
