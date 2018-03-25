use graphql::parsable_as_gsdl_item_list::ParsableAsGsdlItemList;
use graphql::scheme::UnprocessedSource;

mod data;
mod parsable_as_gsdl_item_list;
mod scheme;

mod generated_lalrpop;

pub fn parse_gsdl(source: String) -> Result<scheme::Processed, Vec<String>> {
    source.parse_as_gsdl_item_list()?.build_gsdl()?.process()
}

#[cfg(test)]
mod tests;
