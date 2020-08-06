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
                    <p>
                        <b>Selected volume: {{ volume }}</b>
                    </p>
                    <progress-bar :startTime="volume * 5" :running="running" />
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import ProgressBar from "@/components/ProgressBar.vue";

// TEsting purpose, 1ml = 5s.

export default {
    name: "Step3",
    props: ["recipe", "volume", "running"],
    components: {
        ProgressBar
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
