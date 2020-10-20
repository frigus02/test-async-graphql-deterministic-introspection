use async_graphql::{
    Context, EmptySubscription, FieldResult, MergedObject, Object, Request, Schema, SimpleObject,
    ID,
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
    async fn fruit(&self, context: &Context<'_>, id: ID) -> FieldResult<Fruit> {
        Ok(self.get_fruit(context, id).await?)
    }

    #[graphql(entity)]
    async fn get_fruit(&self, _context: &Context<'_>, id: ID) -> FieldResult<Fruit> {
        Ok(Fruit {
            id,
            name: "Apple".into(),
        })
    }
}

#[derive(Default)]
struct FruitMutation;

#[Object]
impl FruitMutation {
    async fn eat_fruit(&self, _context: &Context<'_>, id: ID) -> FieldResult<Fruit> {
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
    async fn vegetable(&self, _context: &Context<'_>, id: ID) -> Vegetable {
        Vegetable {
            id,
            name: "Carrot".into(),
        }
    }
}

#[derive(MergedObject, Default)]
struct Query(FruitQuery, VegetableQuery);

#[derive(MergedObject, Default)]
struct Mutation(FruitMutation);

#[async_std::main]
async fn main() {
    let schema = Schema::new(Query::default(), Mutation::default(), EmptySubscription);
    let query = include_str!("./introspection_query.graphql");
    let request = Request::new(query);
    let response = schema.execute(request).await;
    println!("{}", serde_json::to_string_pretty(&response.data).unwrap());
}
