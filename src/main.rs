use async_graphql::{
    EmptyMutation, EmptySubscription, MergedObject, Object, Request, Schema, SimpleObject, ID,
};

#[derive(SimpleObject)]
struct Fruit {
    id: ID,
    name: String,
}

#[derive(SimpleObject)]
struct Vegetable {
    id: ID,
    name: String,
}

#[derive(Default)]
struct FruitQuery;

#[Object]
impl FruitQuery {
    #[graphql(entity)]
    async fn get_fruit(&self, id: ID) -> Fruit {
        Fruit {
            id,
            name: "Apple".into(),
        }
    }
}

#[derive(Default)]
struct VegetableQuery;

#[Object]
impl VegetableQuery {
    #[graphql(entity)]
    async fn get_vegetable(&self, id: ID) -> Vegetable {
        Vegetable {
            id,
            name: "Carrot".into(),
        }
    }
}

#[derive(MergedObject, Default)]
struct Query(FruitQuery, VegetableQuery);

#[async_std::main]
async fn main() {
    let schema = Schema::new(Query::default(), EmptyMutation, EmptySubscription);
    let query = include_str!("./introspection_query.graphql");
    let request = Request::new(query);
    let response = schema.execute(request).await;
    println!("{}", serde_json::to_string_pretty(&response.data).unwrap());
}
