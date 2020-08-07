// Middlewares

// Pages
import HomePage from "@/pages/Homepage.vue";
import Settings from "@/pages/Settings.vue";

export default [
    {
        path: "/",
        name: "homepage",
        component: HomePage
    },
    {
        path: "/settings",
        name: "settings",
        component: Settings
    }
];
