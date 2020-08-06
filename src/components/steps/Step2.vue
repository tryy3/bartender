<template>
    <div class="recipe">
        <div class="card">
            <div class="header">
                <div class="image">
                    <cld-image
                        :publicId="recipe.image || '/samples/animals/cat.jpg'"
                        secure="true"
                    >
                        <cld-transformation
                            width="350"
                            height="350"
                            crop="fill"
                        />
                    </cld-image>
                </div>
                <div class="description">
                    <h1>{{ recipe.title }}</h1>
                    <p>{{ recipe.description }}</p>
                    adasd<br />
                    adasdsa<br />
                    hgfdgds<br />
                </div>
            </div>

            <h2>Select the desired volume</h2>
            <div class="size">
                <volume-selector @volumeSelected="volumeSelected" />
            </div>
        </div>
    </div>
</template>

<script>
import VolumeSelector from "@/components/VolumeSelector.vue";

export default {
    name: "Step2",
    components: { VolumeSelector },
    props: ["recipe"],
    methods: {
        volumeSelected(v) {
            this.$emit("volumeSelected", v);
        }
    }
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.recipe {
    text-align: left;
    margin: 16px 16px;
}

h2 {
    margin: 0;
    margin-bottom: 7px;
}

.card {
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.12);
    border-radius: 8px;
    margin: 6px;
    padding: 16px;
    background-color: #fff;
}
.header {
    display: grid;

    grid-template-columns: 350px auto;
    margin-bottom: 16px;
}
.header .description {
    margin: 0 16px;
}

.card h1 {
    font-size: 2.5rem;
    line-height: 2.5rem;
    margin: 0;
    margin-bottom: 4px;
}

.card p {
    margin: 0;
}
</style>
