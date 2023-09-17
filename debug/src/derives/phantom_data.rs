use syn::spanned::Spanned;
use syn::{
    AngleBracketedGenericArguments, Field, GenericArgument, Path, PathArguments, PathSegment, Type,
    TypePath,
};

fn parse_generic_arguments(field: &Field) -> Result<Option<String>, syn::Error> {
    if let Type::Path(TypePath {
        path: Path { ref segments, .. },
        ..
    }) = field.ty
    {
        if let Some(ps) = segments.last() {
            if ps.ident == "PhantomData" {
                if let PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    ref args,
                    ..
                }) = ps.arguments
                {
                    if let Some(GenericArgument::Type(Type::Path(TypePath { path, .. }))) =
                        args.first()
                    {
                        if let Some(PathSegment { ident, .. }) = path.segments.first() {
                            return Ok(Some(ident.to_string()));
                        }
                    }
                }
                return Err(syn::Error::new(
                    field.span(),
                    "parsing the type PhantomData failed",
                ));
            }
        }
    }
    Ok(None)
}
pub fn collect_generic_arguments(fields: &[&Field]) -> Result<Vec<String>, syn::Error> {
    let mut res: Vec<String> = Vec::new();
    for &field in fields {
        if let Some(generic_argument) = parse_generic_arguments(field)? {
            res.push(generic_argument);
        }
    }
    Ok(res)
}
