import { createApp } from "vue";
import { createPinia } from "pinia";
import naive from "naive-ui";
import router from "./router";
import i18n from "./i18n";
import App from "./App.vue";
import "./index.css";

const app = createApp(App);
app.use(createPinia());
app.use(naive);
app.use(i18n);
app.use(router);
app.mount("#app");
