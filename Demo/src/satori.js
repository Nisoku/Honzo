import { createSatori } from "@nisoku/satori";

export const satori = createSatori({
  logLevel: "debug",
  enableConsole: true,
  enableCallsite: true,
  pollingInterval: 100,
});

export const bookLog = satori.createLogger("book");
export const settingsLog = satori.createLogger("settings");
export const makerLog = satori.createLogger("maker");
