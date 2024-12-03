import { defineConfig } from "vitest/config";
import { sveltekit } from "@sveltejs/kit/vite";
import { svelteTesting } from "@testing-library/svelte/vite";

export default defineConfig({
  plugins: [sveltekit(), svelteTesting()],

  test: {
    include: ["src/**/*.{test,spec}.{js,ts}"],
    exclude: ["src/tests/e2e"],
    environment: "jsdom",
    setupFiles: ["./vitest-setup.ts"]
  }
});
