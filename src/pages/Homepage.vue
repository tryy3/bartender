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
            <button class="wizard-button" type="primary" slot="prev">Previous</button>
            <button class="wizard-button" type="primary" slot="finish">Finish</button>
            <button class="wizard-button" :disabled="!isCurrentStepCompleted" type="primary" slot="next">Next</button>
        </form-wizard>
    </div>
</template>

<script>
import Step1 from "@/pages/steps/Step1.vue";
import Step2 from "@/pages/steps/Step2.vue";
import Step3 from "@/pages/steps/Step3.vue";

import { promisified } from "tauri/api/tauri";

export default {
    name: "Homepage",
    components: {
        Step1,
        Step2,
        Step3,
    },
    data() {
        return {
            selectedRecipe: "",
            selectedVolume: "",
            firstStepError: "",
            secondStepError: "",
            running: false,
            loading: false,
            recipes: [],
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
        },
        isCurrentStepCompleted() {
            let index = 0;
            if (typeof this.$refs.form == "undefined") index = 0;
            else if (typeof this.$refs.form.activeTabIndex == "undefined") index = 0;
            else index = this.$refs.form.activeTabIndex;

            if (index == 0) {
                return this.selectedRecipe != "" && this.firstStepError == "";
            } else if (index == 1) {
                return this.selectedVolume != "" && this.secondStepError == "";
            }
            return false
        },
    },
    created() {
        this.loadRecipes();
    },
    methods: {
        loadRecipes() {
            this.loading = true;
            promisified({
                cmd: "getRecipes"
            }).then(resp => {
                this.loading = false;
                this.recipes = resp;
            }).catch(err => {
                this.loading = false;
                this.$toasted.error(err, { duration: 3000 });
            })
        },
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

.wizard-button {
    border: 4px double rgb(231, 76, 60);
    border-radius: 60px;
    color: #fff;
    background: rgb(231, 76, 60);
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

.wizard-button:disabled, .wizard-button:disabled:hover {
    background: #adb5bd;
    border-color: #adb5bd;
    color: #495057;
    cursor: default;
}

.wizard-button:hover {
    background: transparent;
    color: rgba(231, 76, 60);
}

.wizard-button:focus,.wizard-button:active {
    outline: 0;
}
</style>

<style>
.vue-form-wizard .wizard-header {
    padding: 0;
}
div[role=tab]:focus, span[role=button]:focus {
    outline: 0;
}
</style>
