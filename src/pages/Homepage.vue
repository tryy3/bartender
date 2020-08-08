<template>
    <div class="home">
        <form-wizard @on-complete="onComplete" ref="form" title="" subtitle="">
            <tab-content
                title="Choose a Recipe"
                :before-change="validateFirstStep"
            >
                <div class="alert" v-if="firstStepError != ''">
                    <b>Error!</b> {{ firstStepError }}
                </div>
                <step1
                    :recipes="recipes"
                    @recipeSelected="firstStepSelection"
                />
            </tab-content>
            <tab-content
                title="Choose the volume"
                :before-change="validateSecondStep"
            >
                <div class="alert" v-if="secondStepError != ''">
                    <b>Error!</b> {{ secondStepError }}
                </div>
                <step2 :recipe="recipe" @volumeSelected="secondStepSelection">
                </step2>
            </tab-content>
            <tab-content title="Confirmation">
                <step3
                    :recipe="recipe"
                    :volume="selectedVolume"
                    :running="running"
                />
            </tab-content>
        </form-wizard>
    </div>
</template>

<script>
import Step1 from "@/pages/steps/Step1.vue";
import Step2 from "@/pages/steps/Step2.vue";
import Step3 from "@/pages/steps/Step3.vue";

import { promisified } from "tauri/api/tauri";

import gql from "graphql-tag";

export default {
    name: "Homepage",
    components: {
        Step1,
        Step2,
        Step3
    },
    data() {
        return {
            selectedRecipe: "yARypNrS",
            selectedVolume: "100",
            firstStepError: "",
            secondStepError: "",
            running: false
        };
    },
    computed: {
        recipe() {
            let defaultRecipe = {
                id: "",
                title: "",
                image: "",
                description: ""
            };
            if (typeof this.recipes == "undefined") return defaultRecipe;

            let recipe = this.recipes.filter(
                o => o.id == this.selectedRecipe
            )[0];

            if (typeof recipe == "undefined") return defaultRecipe;
            return recipe;
        }
    },
    apollo: {
        recipes: gql`
            query {
                recipes {
                    description
                    id
                    image
                    title
                    recipeIngredients {
                        id
                        ingredientId
                        measurementType
                        measurementValue
                    }
                }
            }
        `
    },
    methods: {
        onComplete: function() {
            this.running = true;
            let payload = {
                pumps: [
                    {
                        id: "123",
                        volume: 3
                    },
                    {
                        id: "321",
                        volume: 9
                    }
                ]
            };
            promisified({
                cmd: "pourDrink",
                payload
            })
                .then(response => {
                    this.$toasted.success(response.message, { duration: 3000 });

                    // Reset all steps and values
                    this.selectedRecipe = "";
                    this.selectedVolume = "";
                    this.firstStepError = "";
                    this.secondStepError = "";
                    this.running = false;
                    this.$refs.form.reset();
                })
                .catch(error => {
                    // do something with the Err() response string
                    alert(error);
                });
        },
        validateFirstStep() {
            return new Promise(resolve => {
                if (this.selectedRecipe == "") {
                    this.firstStepError =
                        "Please select a recipe before continuing";
                    resolve(false);
                } else {
                    this.firstStepError = "";
                    resolve(true);
                }
            });
        },
        validateSecondStep() {
            return new Promise(resolve => {
                if (this.selectedVolume == "") {
                    this.secondStepError =
                        "Please select a your desired volume";
                    resolve(false);
                } else {
                    this.secondStepError = "";
                    resolve(true);
                }
            });
        },
        firstStepSelection(v) {
            this.selectedRecipe = v;
        },
        secondStepSelection(v) {
            this.selectedVolume = v;
        }
    }
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.home {
    margin: 16px 4px;
}
/* The alert message box */
.alert {
    padding: 20px;
    background-color: #f44336; /* Red */
    color: white;
    margin-bottom: 15px;
    text-align: left;
    display: inline-block;
}
</style>

<style>
.vue-form-wizard .wizard-header {
    padding: 0;
}
</style>
