use syn::{AttributeArgs, Lit, LitStr, NestedMeta};

use crate::error::Error;

#[derive(Default)]
pub struct Args {
    // The name of the generated function
    pub name: Option<String>,
    // The doc-string of the generated function
    pub docs: Option<String>,
}

impl Args {
    pub fn set_name(&mut self, name: String) -> Result<(), Error> {
        if self.name.is_some() {
            Err(Error::ConflictingArgs)
        } else {
            self.name = Some(name);
            Ok(())
        }
    }

    pub fn set_docs(&mut self, docs: String) -> Result<(), Error> {
        if self.docs.is_some() {
            Err(Error::ConflictingArgs)
        } else {
            self.docs = Some(docs);
            Ok(())
        }
    }
}

enum ArgKind {
    Docs(LitStr),
    Name(LitStr),
}

fn literal_into_string_literal(literal: Lit) -> Result<LitStr, Error> {
    if let Lit::Str(lit_str) = literal {
        Ok(lit_str)
    } else {
        Err(Error::StringLiteralExpected)
    }
}

fn parse_meta_into_string_literal(nested_meta: NestedMeta) -> Result<ArgKind, Error> {
    let meta = match nested_meta {
        NestedMeta::Meta(meta) => meta,
        NestedMeta::Lit(_) => Err(Error::NestedMetaExpected)?,
    };

    let pair = match meta {
        syn::Meta::NameValue(pair) => pair,
        _ => Err(Error::NameValueExpected)?,
    };

    if pair.path.is_ident("docs") {
        Ok(ArgKind::Docs(literal_into_string_literal(pair.lit)?))
    } else if pair.path.is_ident("name") {
        Ok(ArgKind::Name(literal_into_string_literal(pair.lit)?))
    } else {
        Err(Error::UnexpectedName)
    }
}

pub fn parse_args(attrib_args: AttributeArgs) -> Result<Args, Error> {
    let mut args = Args::default();

    if attrib_args.is_empty() {
        return Ok(args);
    }

    let literals: Result<Vec<_>, _> = attrib_args
        .into_iter()
        .map(parse_meta_into_string_literal)
        .collect();

    let literals = literals?;

    if literals.len() > 2 {
        Err(Error::TooManyArgsSupplied)?;
    }

    for literal in literals {
        match literal {
            ArgKind::Docs(docs) => args.set_docs(docs.value())?,
            ArgKind::Name(name) => args.set_name(name.value())?,
        }
    }

    Ok(args)
}
