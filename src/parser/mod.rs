use std::fmt;

use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};

#[cfg(test)]
mod tests;

// TODO
struct ListDefinitions {}

struct ListDefinitionsVisitor {}

impl ListDefinitionsVisitor {
    fn new() -> Self {
        ListDefinitionsVisitor {}
    }
}

impl<'de> Visitor<'de> for ListDefinitionsVisitor {
    // Our Visitor is going to produce a RuntimeGraph.
    type Value = ListDefinitions;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("List definitions")
    }

    fn visit_map<A>(self, _map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        Ok(ListDefinitions {})
    }
}

impl<'de> Deserialize<'de> for ListDefinitions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Instantiate our Visitor and ask the Deserializer to drive
        // it over the input data, resulting in an instance of ListDefinitions.
        deserializer.deserialize_map(ListDefinitionsVisitor::new())
    }
}
