import { Args } from "@oclif/core";
import { BaseCommand } from "../base-command.js";

export default class Fix extends BaseCommand {
  public static readonly summary = "Fix the project issues";
  public static readonly description =
    "Fix the project issues related to security, quality and config problems.";

  public static readonly args = {
    type: Args.string({
      name: "type",
      required: true,
      description: "What type of issue to search for",
      hidden: false,
      default: "security",
      options: ["security", "quality", "config", "all"],
    }),
  };

  public static readonly examples = [
    "<%= config.bin %> <%= command.id %> security",
    "<%= config.bin %> <%= command.id %> quality",
    "<%= config.bin %> <%= command.id %> config",
    "<%= config.bin %> <%= command.id %> all",
  ];

  public async run(): Promise<void> {
    const { args } = await this.parse(Fix);
    this.log(args.type);
  }
}
