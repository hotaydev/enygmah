#!/usr/bin/env node
(await import("../dist/flags/debug.js")).preprocessCliFlags(process);

async function main() {
  const { execute } = await import("@oclif/core");
  await execute({ dir: import.meta.url });
}

await main();
