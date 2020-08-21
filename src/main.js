// Import Vue related imports
import Vue from "vue";
import VueRouter from "vue-router";
import Cloudinary from "cloudinary-vue";
import VueFormWizard from "vue-form-wizard";
import VueEllipseProgress from "vue-ellipse-progress";
import VueSelect from "vue-select";
import VueToasted from "vue-toasted";

// Import Vue css
import "vue-form-wizard/dist/vue-form-wizard.min.css";
import "vue-select/dist/vue-select.css";

// Internal Imports
import routes from "./routes/";

// Modules and App
import App from "./App.vue";
import { library } from "@fortawesome/fontawesome-svg-core";
import { faSearch, faCog } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";

// Add access to specific fontawesome icons
library.add(faSearch, faCog);

// Add components to Vue
Vue.component("font-awesome-icon", FontAwesomeIcon);
Vue.component("v-select", VueSelect);

// Add vue plugins
Vue.use(VueToasted);
Vue.use(VueRouter);
Vue.use(VueFormWizard);
Vue.use(VueEllipseProgress);
Vue.use(Cloudinary, {
    configuration: {
        cloudName: "ddsiiisuy"
    }
});

// Initialize the router
const router = new VueRouter({
    mode: "history",
    routes
});

// Development/Production settings
Vue.config.productionTip = false;

new Vue({
    router,
    render: h => h(App)
}).$mount("#app");
