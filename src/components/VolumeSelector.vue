<template>
    <div class="volume-selector">
        <div v-for="volume in volumes" :key="volume">
            <div
                class="selector"
                @click.capture="selectVolume(volume)"
                :data="volume"
                :class="{ selected: volume == value }"
            >
                <h5>{{ volume }} ml</h5>
                <div class="glass">
                    <div class="water" :style="{ height: `${volume}%` }"></div>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
export default {
    name: "Volume-Selector",
    data() {
        return {
            // TODO: Add custom
            volumes: [100, 30, 10, 6, 4, 2],
            value: 0
        };
    },
    methods: {
        selectVolume(e) {
            this.value = e;
            this.$emit("volumeSelected", e);
        }
    }
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.volume-selector {
    display: grid;
    grid-template-columns: repeat(6, 75px);
}

.glass {
    height: 65px;
    width: 35px;
    position: relative;
    border-style: none solid solid solid;
    border-width: 5px;
    border-color: lightgray;
    border-radius: 5px;
}

.water {
    width: 100%;
    height: 0;
    background-color: skyblue;
    position: absolute;
    bottom: 0;
}

.selector:hover .glass {
    box-shadow: 0 0 7px 1px rgba(41, 128, 185, 0.3);
}

.selector.selected .glass {
    box-shadow: 0 0 7px 1px rgba(41, 128, 185, 1);
}

.selector.selected h5 {
    text-decoration: underline;
}

.selector h5 {
    font-size: 0.9rem;
    margin: 0;
    margin-bottom: 5px;
}

.selector {
    cursor: pointer;
}
</style>
