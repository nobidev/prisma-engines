use crate::{
    ast::{self, IndentationType, NewlineType, WithDocumentation},
    types,
    walkers::{newline, Walker},
};

/// An `enum` declaration in the schema.
pub type EnumWalker<'db> = Walker<'db, crate::EnumId>;
/// One value in an `enum` declaration in the schema.
pub type EnumValueWalker<'db> = Walker<'db, (crate::EnumId, usize)>;

impl<'db> EnumWalker<'db> {
    fn attributes(self) -> &'db types::EnumAttributes {
        &self.db.types.enum_attributes[&self.id]
    }

    /// The name of the enum.
    pub fn name(self) -> &'db str {
        &self.ast_enum().name.name
    }

    /// The AST node.
    pub fn ast_enum(self) -> &'db ast::Enum {
        &self.db.asts[self.id]
    }

    /// The database name of the enum.
    pub fn database_name(self) -> &'db str {
        self.mapped_name().unwrap_or_else(|| self.name())
    }

    /// The mapped name of the enum:
    ///
    /// ```ignore
    /// enum Colour {
    ///     RED
    ///     GREEN
    ///     BLUE
    ///
    ///     @@map("Color")
    ///           ^^^^^^^
    /// }
    /// ```
    pub fn mapped_name(self) -> Option<&'db str> {
        self.attributes().mapped_name.map(|id| &self.db[id])
    }

    /// The values of the enum.
    pub fn values(self) -> impl ExactSizeIterator<Item = EnumValueWalker<'db>> {
        (0..self.ast_enum().values.len()).map(move |idx| self.walk((self.id, idx)))
    }

    /// How fields are indented in the enum.
    pub fn indentation(self) -> IndentationType {
        IndentationType::default()
    }

    /// What kind of newlines the enum uses.
    pub fn newline(self) -> NewlineType {
        let value = match self.ast_enum().values.last() {
            Some(value) => value,
            None => return NewlineType::default(),
        };

        let src = self.db.source(self.id.0);

        newline(src, value.span)
    }

    /// The name of the schema the enum belongs to.
    ///
    /// ```ignore
    /// @@schema("public")
    ///          ^^^^^^^^
    /// ```
    pub fn schema(self) -> Option<(&'db str, ast::Span)> {
        self.attributes().schema.map(|(id, span)| (&self.db[id], span))
    }
}

impl<'db> EnumValueWalker<'db> {
    fn r#enum(self) -> EnumWalker<'db> {
        self.walk(self.id.0)
    }

    /// The enum documentation
    pub fn documentation(self) -> Option<&'db str> {
        self.r#enum().ast_enum().values[self.id.1].documentation()
    }

    /// The name of the value.
    pub fn name(self) -> &'db str {
        &self.r#enum().ast_enum().values[self.id.1].name.name
    }

    /// The database name of the enum.
    pub fn database_name(self) -> &'db str {
        self.mapped_name().unwrap_or_else(|| self.name())
    }

    /// The mapped name of the value:
    ///
    /// ```ignore
    /// enum Colour {
    ///     RED @map("scarlet")
    ///     GREEN @map("salad")
    ///                ^^^^^^^
    ///     BLUE @map("schmurf")
    /// }
    /// ```
    pub fn mapped_name(self) -> Option<&'db str> {
        self.db.types.enum_attributes[&self.id.0]
            .mapped_values
            .get(&(self.id.1 as u32))
            .map(|id| &self.db[*id])
    }
}
