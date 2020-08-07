<template>
    <div class="recipe">
        <div class="card">
            <div class="header">
                <h1>Settings</h1>
            </div>
            <div class="body">
                <template v-if="loading || $apollo.loading">
                    Loading data...
                </template>
                <template v-else>
                    <div class="pumps">
                        <div v-for="pump in settings.pumps" :key="pump.id">
                            <pump :options="ingredient_options" :pump="pump" />
                        </div>
                    </div>
                </template>
            </div>
            <button @click="onSave">Save settings</button>
        </div>
    </div>
</template>

<script>
import gql from "graphql-tag";
import { promisified } from "tauri/api/tauri";
import Pump from "@/components/Pump.vue";
import { cloneDeep } from "lodash";

export default {
    name: "Recipe",
    components: { Pump },
    data() {
        return {
            loading: false,
            settings: {}
        };
    },
    computed: {
        ingredient_options() {
            return this.ingredients.map(o => {
                return { label: o.title, id: o.id };
            });
        }
    },
    created() {
        this.loadSettings();
    },
    methods: {
        loadSettings() {
            this.loading = true;
            promisified({
                cmd: "loadSettings"
            })
                .then(resp => {
                    this.loading = false;
                    this.settings = resp;
                })
                .catch(err => {
                    this.loading = false;
                    this.$toasted.error(err, { duration: 3000 });
                });
        },
        onSave() {
            let settings = cloneDeep(this.settings);
            promisified({
                cmd: "saveSettings",
                payload: settings
            })
                .then(resp => {
                    this.$toasted.success(resp.message, { duration: 3000 });
                })
                .catch(err => {
                    this.$toasted.error(err, { duration: 3000 });
                });
        }
    },
    apollo: {
        ingredients: {
            query: gql`
                query {
                    ingredients {
                        description
                        image
                        id
                        title
                    }
                }
            `
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
.card button {
    border: 4px double #2ecc71;
    border-radius: 60px;
    color: #fff;
    background: #2ecc71;
    transition: background-color 0.35s ease;
    font-size: 0.85rem;
    cursor: pointer;
    padding: 10px 15px;
    display: inline-block;
    margin: 15px 30px;
    letter-spacing: 1px;
    font-weight: 700;
    outline: none;
    position: relative;
}

button:hover {
    background: transparent;
    color: #2ecc71;
}
</style>
