import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import { initTheme } from "./lib/theme";

initTheme();
createApp(App).mount("#app");
