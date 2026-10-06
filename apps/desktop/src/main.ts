import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import { i18n, initI18n } from "./lib/i18n";
import { initTheme } from "./lib/theme";

initTheme();
initI18n();
createApp(App).use(i18n).mount("#app");
