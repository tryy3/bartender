<template>
    <div class="pump">
        <h2>Pump {{ pump.id }}</h2>
        <v-select :options="options" v-model="selected" />
        {{ pump }}
        {{ selected }}
    </div>
</template>

<script>
export default {
    name: "Recipe",
    props: ["options", "pump"],
    computed: {
        selected: {
            get() {
                return this.options.find(o => o.id == this.pump.ingredient_id);
            },
            set(v) {
                this.pump.ingredient_id = v.id;
            }
        }
    }
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.card {
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.12);
    border-radius: 8px;
    margin: 6px;
    padding: 16px;
    background-color: #fff;
}
.pumps {
    display: grid;

    grid-template-columns: repeat(4, auto);
    margin-bottom: 16px;
    text-align: left;
}
.pump {
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.12);
    height: 200px;
    margin: 16px;
    padding: 8px;
}

.pump h2 {
    font-size: 2rem;
    margin: 0;
    margin-bottom: 5px;
}
</style>
