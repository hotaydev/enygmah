import { BaseCommand } from "../base-command.js";

export default class Init extends BaseCommand {
  static description = "Configure the tool - long description";
  static summary = "Configure the tool";

  static examples = ["<%= config.bin %> <%= command.id %>"];

  public async run(): Promise<void> {
    // Configure the tool with an .enygmah file and a folder for hooks and other configuration
  }
}
