// Middlewares

// Pages
import HomePage from "@/components/Homepage.vue";
import Recipe from "@/pages/Recipe.vue";
import Settings from "@/pages/Settings.vue";

export default [
    {
        path: "/",
        name: "homepage",
        component: HomePage
    },
    {
        path: "/recipe/:ID",
        name: "recipe",
        component: Recipe
    },
    {
        path: "/settings",
        name: "settings",
        component: Settings
    }
];
