// Implemented tis way to call the `preprocessCliFlags` before the real run of the CLI.
// To use global flags, refer to this documentation: https://oclif.io/docs/global_flags

/**
 * Interface used to define what is the type of the `process` without needing to import something else
 */
export interface ProcessLike {
  argv: string[];
  env: { [key: string]: string | undefined };
}

/**
 * Process CLI flags like `--debug` before any other thing
 * @param process The process in where the program is being executed
 */
export function preprocessCliFlags(process: ProcessLike): void {
  for (const arg of process.argv) {
    if (arg === "--debug") {
      process.env.DEBUG = "*";
      process.argv.splice(process.argv.indexOf("--debug"), 1);
    }
  }
}
