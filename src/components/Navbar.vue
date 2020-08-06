<template>
    <nav class="navbar is-transparent">
        <div class="navbar-brand">
            <a class="navbar-item" href="https://bulma.io">
                <img
                    src="https://bulma.io/images/bulma-logo.png"
                    alt="Bulma: a modern CSS framework based on Flexbox"
                    width="112"
                    height="28"
                />
            </a>
            <div
                class="navbar-burger burger"
                data-target="navbarExampleTransparentExample"
            >
                <span></span>
                <span></span>
                <span></span>
            </div>
        </div>

        <div id="navbarExampleTransparentExample" class="navbar-menu">
            <div class="navbar-start">
                <a class="navbar-item" href="/">
                    Home
                </a>
            </div>

            <div class="navbar-end">
                <a class="navbar-item" href="/">
                    <font-awesome-icon icon="cog" />
                </a>
                <a class="navbar-item" href="/">
                    <font-awesome-icon icon="search" />
                </a>
            </div>
        </div>
    </nav>
</template>

<script>
export default {
    name: "Navbar"
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.navbar {
    min-height: 3.25rem;
    position: relative;
    z-index: 30;
    background-color: #f5f5f5;
    color: rgba(0, 0, 0, 0.7);
    box-shadow: 0 2px 0 0 #f5f5f5;
}

.navbar,
.navbar-end,
.navbar-menu,
.navbar-start {
    align-items: stretch;
    display: flex;
}

.navbar-brand {
    align-items: stretch;
    display: flex;
    flex-shrink: 0;
    min-height: 3.25rem;
}

.navbar-menu {
    flex-grow: 1;
    flex-shrink: 0;
}

.navbar-start {
    justify-content: flex-start;
    margin-right: auto;
}

.navbar-item,
.navbar-link {
    align-items: center;
    display: flex;
    cursor: pointer;
    color: #4a4a4a;
    line-height: 1.5;
    padding: 0.5rem 0.75rem;
    position: relative;
    text-decoration: none;
}

.navbar-link:hover,
.navbar-item:hover {
    color: #3237dc;
}

.navbar-item {
    flex-grow: 0;
    flex-shrink: 0;
}
</style>
