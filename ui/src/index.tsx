const client = require("./client");
const http = require("http");

const host = "localhost";
const port = 8080;

const startServer = () => {
  try {
    const requestListener = function (req, res) {};

    const server = http.createServer(requestListener);
    server.listen(port, host, () => {
      console.log(`Server is running on http://${host}:${port}`);
    });
  } catch (err) {
    console.log(err);
  }
};

startServer();
