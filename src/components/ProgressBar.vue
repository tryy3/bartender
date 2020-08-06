<template>
    <div
        link="https://github.com/setaman/vue-ellipse-progress/blob/demo/demo/src/components/Examples/Example5.vue"
    >
        <component
            :is="component"
            id="timer-example"
            :progress="progress"
            :determinate="determinate"
            color="#7579ff"
            empty-color="#324c7e"
            :emptyColorFill="emptyColorFill"
            thickness="2%"
            emptyThickness="5%"
            :size="180"
            dash="strict 60 0.8"
            lineMode="in-over"
            :legend="false"
            legendClass="legend-custom-style"
            fontSize="1.5rem"
            font-color="white"
            animation="loop 1000 100"
            :loading="loading"
            :no-data="noData"
        >
            <span slot="legend-caption">
                <span>{{ minPrefix }}{{ min }}</span>
                <span class="mx-2">:</span>
                <span>{{ secPrefix }}{{ sec }}</span>
            </span>
        </component>
    </div>
</template>

<script>
export default {
    name: "ProgressBar",
    props: ["startTime", "running"],
    data() {
        return {
            loading: false,
            noData: false,
            determinate: false,
            time: 0,
            tasksDone: 125,
            emptyColor: {
                radial: false,
                colors: [
                    {
                        color: "#8ec5fc",
                        offset: "0",
                        opacity: "1"
                    },
                    {
                        color: "#e0c3fc",
                        offset: "100",
                        opacity: "1"
                    }
                ]
            },
            emptyColorFill: {
                radial: true,
                colors: [
                    {
                        color: "#3260FC",
                        offset: "50",
                        opacity: "0.2"
                    },
                    {
                        color: "#3260FC",
                        offset: "50",
                        opacity: "0.15"
                    },
                    {
                        color: "#3260FC",
                        offset: "70",
                        opacity: "0.15"
                    },
                    {
                        color: "#3260FC",
                        offset: "70",
                        opacity: "0.1"
                    },
                    {
                        color: "#3260FC",
                        offset: "90",
                        opacity: "0.1"
                    },
                    {
                        color: "transparent",
                        offset: "90",
                        opacity: "0.1"
                    },
                    {
                        color: "transparent",
                        offset: "95",
                        opacity: "0.1"
                    },
                    {
                        color: "transparent",
                        offset: "95",
                        opacity: "0.1"
                    }
                ]
            }
        };
    },
    computed: {
        tasksDonePercent() {
            return (this.tasksDone * 100) / 200;
        },
        minPrefix() {
            return this.min < 10 ? "0" : "";
        },
        secPrefix() {
            return this.sec < 10 ? "0" : "";
        },
        component() {
            return this.test
                ? "vue-ellipse-progress-test"
                : "vue-ellipse-progress";
        },
        timeLeft() {
            return this.startTime - this.time;
        },
        sec() {
            return this.timeLeft % 60;
        },
        min() {
            return Math.floor(this.timeLeft / 60);
        },
        progress() {
            return Math.floor(((this.time + 1) / this.startTime) * 100);
        }
    },
    methods: {
        runTimer() {
            if (!this.running) return;
            if (this.time == this.startTime) {
                this.time = 0;
            }
            this.time++;
        }
    },
    mounted() {
        setInterval(this.runTimer, 1000);
    }
};
</script>

<style scoped></style>
