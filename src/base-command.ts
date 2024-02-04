import { Command, Flags } from "@oclif/core";

export abstract class BaseCommand extends Command {
  static baseFlags = {
    debug: Flags.boolean({
      // The real debug mode is implemented in ./flags/debug.ts
      // This flag is used just to show the "--debug" flag on the help menu
      description: "Enables debug mode",
    }),
  };
}
