import * as React from 'react'
import { renderToStaticMarkup } from 'react-dom/server';

const http = require("http");

const host = "localhost";
const port = 8080;

const startServer = () => {
  try {
    const requestListener = function (req, res) {
      res.setHeader("Content-Type", "text/html");
      res.writeHead(200);
      res.end(`<!DOCTYPE html><html><body><h1>${renderToStaticMarkup(<span>aaaa</span>)}</h1></body></html>`);
    };

    const server = http.createServer(requestListener);
    server.listen(port, host, () => {
      console.log(`Server is running on http://${host}:${port}`);
    });
  } catch (err) {
    console.log(err);
  }
};

startServer();
