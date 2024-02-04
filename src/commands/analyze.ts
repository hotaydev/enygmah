import { Args } from "@oclif/core";
import { BaseCommand } from "../base-command.js";

export default class Analyze extends BaseCommand {
  static summary = "Analyzes the project in search of issues";
  static description =
    "Analyzes the project for security, quality and config issues.";

  static args = {
    type: Args.string({
      name: "type",
      required: true,
      description: "What type of issue to search for",
      hidden: false,
      default: "security",
      options: ["security", "quality", "config", "all"],
    }),
  };

  static examples = [
    "<%= config.bin %> <%= command.id %> security",
    "<%= config.bin %> <%= command.id %> quality",
    "<%= config.bin %> <%= command.id %> config",
    "<%= config.bin %> <%= command.id %> all",
  ];

  public async run(): Promise<void> {
    const { args } = await this.parse(Analyze);
    this.log(args.type);
  }
}
