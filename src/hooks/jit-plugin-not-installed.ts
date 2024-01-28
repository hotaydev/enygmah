import { CLIError } from "@oclif/core/lib/errors/index.js";
import { Hook } from "@oclif/core";

const hook: Hook<"jit_plugin_not_installed"> = async function (opts) {
  try {
    await opts.config.runCommand("plugins:install", [
      `${opts.command.pluginName}@${opts.pluginVersion}`,
    ]);
  } catch {
    throw new CLIError(`Could not install plugin ${opts.command.pluginName}`);
  }
};

export default hook;
