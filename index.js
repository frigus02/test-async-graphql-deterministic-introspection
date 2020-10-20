const fs = require("fs");
const path = require("path");
const { buildClientSchema, printSchema } = require("graphql/utilities");
const { validateSchema } = require("graphql/type");

const introspectionSchema = fs.readFileSync(
  path.join(__dirname, "./schema.json"),
  "utf8"
);
const schema = buildClientSchema(JSON.parse(introspectionSchema));
console.log(printSchema(schema));
console.log(validateSchema(schema));
