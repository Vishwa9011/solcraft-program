import type { NextConfig } from "next";
import { resolve } from "path";

const nextConfig: NextConfig = {
  /* config options here */
  turbopack: {
    root: resolve(__dirname),
  },
};

export default nextConfig;
