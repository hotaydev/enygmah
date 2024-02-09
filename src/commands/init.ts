import { BaseCommand } from "../base-command.js";
import path from "node:path";
import { writeFile } from "node:fs";

export default class Init extends BaseCommand {
  public static readonly description = "Configure the tool - long description";
  public static readonly summary = "Configure the tool";

  public static readonly examples = ["<%= config.bin %> <%= command.id %>"];

  public async run(): Promise<void> {
    // Configure the tool with an .enygmah file and a folder for hooks and other configuration
    const config = {
      module: true,
      eslint: true,
      prettier: true,
      hook: true,
      quality: true,
      security: true,
      severity: ["informative", "low", "medium", "high", "critical"],
    };
    // TODO: Save the config file with a different name based on the dev/prod environment, for test purposes
    writeFile(
      path.join(process.cwd(), ".enygmah"),
      JSON.stringify(config, null, 2),
      "utf-8",
      (error) => {
        if (error) {
          this.error(error);
        }
        this.log("Configured with success!");
      },
    );
  }
}
