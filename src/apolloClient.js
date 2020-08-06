import { ApolloClient } from "apollo-client";
import { InMemoryCache } from "apollo-cache-inmemory";
import { createUploadLink } from "apollo-upload-client";
import { setContext } from "apollo-link-context";
import { onError } from "apollo-link-error";
import { ApolloLink } from "apollo-link";

const client = new ApolloClient({
    link: ApolloLink.from([
        setContext(async (_, { headers }) => {
            return {
                headers: {
                    ...headers
                }
            };
        }),

        onError(({ graphQLErrors, networkError }) => {
            if (graphQLErrors)
                graphQLErrors.forEach(({ message, locations, path }) =>
                    console.log(
                        `[GraphQL error]: Message: ${message}, Location: ${locations}, Path: ${path}`
                    )
                );
            if (networkError) console.log(`[Network error]: ${networkError}`);
        }),
        createUploadLink({
            //uri: "https://recipe-maker-backend.tryy3.us/query"
            //uri: "http://localhost:8090/query"
            uri: "https://recipe-maker-backend.herokuapp.com/v1/graphql"
        })
    ]),
    cache: new InMemoryCache()
});

export default client;
