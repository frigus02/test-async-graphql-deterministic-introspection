# Test deterministic introspection query result with async-graphql

In version 1.18.2 of async-graphql the result returned from the introspection query randomly order some fields.

This doesn't seem to be the same anymore for version 2.0.0.

Version 2.0.x seems to have introduced some empty types in the introspection query result. This returns errors in GraphQL schema validation.
