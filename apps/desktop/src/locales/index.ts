import { en } from "./en";
import { messages as zh, type MessageSchema } from "./zh";

export type UiLocale = "zh" | "en";

export const UI_LOCALES: readonly UiLocale[] = ["zh", "en"];

export const messages = { zh, en };

declare module "vue-i18n" {
  // Types `t()` keys and interpolation params across the app.
  export interface DefineLocaleMessage extends MessageSchema {}
}

export type { MessageSchema };
