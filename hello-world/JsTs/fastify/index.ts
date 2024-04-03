import fastify from "fastify";

const server = fastify();
const port = 3000;

server.get("/", async (req, res) => {
  return "Hello World!";
});

server.listen({ port }, (_, address) => {
  console.log(`⚡️[server]: Server is running at ${address}`);
});
