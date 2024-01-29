#!/usr/bin/env -S node --loader ts-node/esm --no-warnings=ExperimentalWarning

(await import("../src/flags/debug.ts")).preprocessCliFlags(process);

async function main() {
  const oclif = await import("@oclif/core");
  oclif.settings.performanceEnabled = true;
  await oclif.execute({ development: true, dir: import.meta.url });
}

await main();
