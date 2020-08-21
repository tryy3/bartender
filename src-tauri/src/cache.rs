pub mod cache {
    use futures::executor::block_on;
    use graphql_client::{GraphQLQuery, Response};
    use serde::Serialize;
    use std::error::Error;

    #[derive(GraphQLQuery, Serialize, Clone)]
    #[graphql(
        schema_path = "graphql/schema.json",
        query_path = "graphql/ingredients.graphql",
        response_derives = "Serialize,Debug,Clone"
    )]
    pub struct IngredientsQuery;

    #[derive(GraphQLQuery, Serialize, Clone)]
    #[graphql(
        schema_path = "graphql/schema.json",
        query_path = "graphql/recipes.graphql",
        response_derives = "Serialize,Debug,Clone"
    )]
    pub struct RecipesQuery;

    async fn query_ingredients() -> Result<Response<ingredients_query::ResponseData>, Box<dyn Error>>
    {
        // this is the important line
        let request_body = IngredientsQuery::build_query(ingredients_query::Variables {});
        let client = reqwest::Client::new();
        let res = client
            .post("https://recipe-maker-backend.herokuapp.com/v1/graphql")
            .json(&request_body)
            .send()
            .await?;
        let response_body: Response<ingredients_query::ResponseData> = res.json().await?;
        Ok(response_body)
    }

    async fn query_recipes() -> Result<Response<recipes_query::ResponseData>, Box<dyn Error>> {
        // this is the important line
        let request_body = RecipesQuery::build_query(recipes_query::Variables {});
        let client = reqwest::Client::new();
        let res = client
            .post("https://recipe-maker-backend.herokuapp.com/v1/graphql")
            .json(&request_body)
            .send()
            .await?;
        let response_body: Response<recipes_query::ResponseData> = res.json().await?;
        Ok(response_body)
    }

    #[derive(Debug, Clone)]
    pub struct Cache {
        pub ingredients: Vec<ingredients_query::IngredientsQueryIngredients>,
        pub recipes: Vec<recipes_query::RecipesQueryRecipes>,
    }

    impl Cache {
        // TODO: This is probablyyyyy not the idomatic way to do this...
        pub fn new() -> Result<Self, Box<dyn Error>> {
            let ingredient_response = match block_on(query_ingredients()) {
                Ok(res) => res,
                Err(e) => return Err(e),
            };

            let recipe_response = match block_on(query_recipes()) {
                Ok(res) => res,
                Err(e) => return Err(e),
            };

            Ok(Self {
                ingredients: ingredient_response.data.unwrap().ingredients,
                recipes: recipe_response.data.unwrap().recipes,
            })
        }
    }
}
