const port = 8000;
const basePath = "./public";

const handler = async (request: Request): Promise<Response> => {
  let pathName = new URL(request.url).pathname;
  if (pathName === "/") {
    pathName = "/index.html";
  }
  const filePath = basePath + pathName;

  let fileSize;
  try {
    fileSize = (await Deno.stat(filePath)).size;
  } catch (e) {
    if (e instanceof Deno.errors.NotFound) {
      return new Response(null, { status: 404 });
    }
    return new Response(null, { status: 500 });
  }
  const body = (await Deno.open(filePath)).readable;

  return new Response(body, {
    headers: {
      "content-length": fileSize.toString(),
    },
  });
};

Deno.serve({ port }, handler);
