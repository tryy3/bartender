<template>
    <div id="app">
        <Navbar />
        <router-view class="view"></router-view>
    </div>
</template>

<script>
import Navbar from "./components/Navbar.vue";

export default {
    name: "App",
    components: {
        Navbar
    }
};
</script>

<!-- https://flatuicolors.com/palette/defo -->

<style>
body {
    margin: 0;
    background: #ecf0f1;
    color: #ecf0f1;
}

#app {
    font-family: Avenir, Helvetica, Arial, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    text-align: center;
    color: #2c3e50;
}

a,
a:link,
a:hover,
a:active {
    color: #34495e;
    text-decoration: none;
}
</style>
