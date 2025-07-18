import { docs } from "@/.source";
import { loader } from "fumadocs-core/source";
import { attachFile, createOpenAPI } from "fumadocs-openapi/server";

// See https://fumadocs.vercel.app/docs/headless/source-api for more info
export const source = loader({
  // it assigns a URL to your pages
  baseUrl: "/",
  source: docs.toFumadocsSource(),
  pageTree: {
    // adds a badge to each page item in page tree
    attachFile,
  },
});

export const openapi = createOpenAPI();
