<template>
    <div class="home">
        <div class="grid">
            <div v-for="recipe in recipes" :key="recipe.id">
                <div
                    class="card"
                    @click="selectRecipe(recipe.id)"
                    :class="{ selected: selectedRecipe == recipe.id }"
                >
                    <div class="image-box">
                        <cld-image
                            :publicId="
                                recipe.image || '/samples/animals/cat.jpg'
                            "
                            secure="true"
                        >
                            <cld-transformation
                                width="206"
                                height="206"
                                crop="thumb"
                            />
                            <cld-transformation radius="20" />
                        </cld-image>
                    </div>
                    <h3 class="title">{{ recipe.title }}</h3>
                    {{ recipe.id }}<br />
                    <p>
                        {{ recipe.description }}
                    </p>
                    <button>Select Recipe</button>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
export default {
    name: "Step1",
    props: ["recipes"],
    data() {
        return {
            selectedRecipe: ""
        };
    },
    methods: {
        selectRecipe(recipe) {
            this.selectedRecipe = recipe;
            this.$emit("recipeSelected", recipe);
        }
    }
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.home {
    margin: 16px 4px;
}

.grid {
    display: grid;
    grid-template-columns: 25% 25% 25% 25%;
}

.card {
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.12);
    border-radius: 8px;
    margin: 6px;
    padding: 16px;
    background-color: #fff;
    cursor: pointer;
}

.card:hover {
    box-shadow: 0 0 7px 1px rgba(41, 128, 185, 0.3);
}

.card.selected {
    box-shadow: 0 0 7px 1px rgba(41, 128, 185, 1);
}

.card h3 {
    font-size: 1.3rem;
    margin: 0;
    margin-bottom: 4px;
}

.card p {
    margin: 0;
}

.card button {
    border: 4px double #2980b9;
    border-radius: 60px;
    color: #fff;
    background: #2980b9;
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

.card:hover button {
    background: transparent;
    color: #2980b9;
}
</style>
