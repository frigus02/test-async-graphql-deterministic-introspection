use async_graphql::{
    EmptyMutation, EmptySubscription, FieldResult, GQLMergedObject, Object, QueryBuilder, Schema,
    SimpleObject, ID,
};

#[SimpleObject(desc = "A fruit")]
struct Fruit {
    id: ID,
    name: String,
}

#[SimpleObject(desc = "A vegetable")]
struct Vegetable {
    id: ID,
    name: String,
}

#[derive(Default)]
struct FruitQuery;

#[Object]
impl FruitQuery {
    #[entity]
    async fn get_fruit(&self, id: ID) -> FieldResult<Fruit> {
        Ok(Fruit {
            id,
            name: "Apple".into(),
        })
    }
}

#[derive(Default)]
struct VegetableQuery;

#[Object]
impl VegetableQuery {
    #[entity]
    async fn get_vegetable(&self, id: ID) -> FieldResult<Vegetable> {
        Ok(Vegetable {
            id,
            name: "Carrot".into(),
        })
    }
}

#[derive(GQLMergedObject, Default)]
struct Query(FruitQuery, VegetableQuery);

#[async_std::main]
async fn main() {
    let schema = Schema::build(Query::default(), EmptyMutation, EmptySubscription).finish();
    let query = include_str!("./introspection_query.graphql");
    let result = QueryBuilder::new(query).execute(&schema).await.unwrap();
    println!("{:#}", result.data);
}
