extern crate regex;

use graphql::data::unprocessed::GsdlItem;
use graphql::generated_lalrpop::parse_Gsdl;

pub trait ParsableAsGsdlItemList {
    fn parse_as_gsdl_item_list(self) -> Result<Vec<GsdlItem>, Vec<String>>;

    // hackish helper methods. In theory, LALRPOP grammar should parse and ignore comments and commas
    // However, comments and commas can appear pretty much anywhere, so listing them explicitly adds
    // a lot of visual noise to .lalrpop file. Therefore, comments and commas are removed manually
    fn cleanup_gsdl(self) -> String;
    fn remove_commas(self) -> String;
    fn remove_comments(self) -> String;
    fn parse_cleaned_gsdl(self) -> Result<Vec<GsdlItem>, Vec<String>>; // wraps lalrpop parse_Gsdl()
}

impl ParsableAsGsdlItemList for String {
    fn parse_as_gsdl_item_list(self) -> Result<Vec<GsdlItem>, Vec<String>> {
        self.cleanup_gsdl().parse_cleaned_gsdl()
    }

    fn cleanup_gsdl(self) -> String {
        self.remove_commas().remove_comments()
    }

    fn remove_commas(self) -> String {
        self.replace(",", " ")
    }

    fn remove_comments(self) -> String {
        let re = regex::Regex::new("#.*").unwrap();
        String::from(re.replace(self.as_str(), ""))
    }
    fn parse_cleaned_gsdl(self) -> Result<Vec<GsdlItem>, Vec<String>> {
        parse_Gsdl(self.as_str()).map_err(|e| vec![format!("{:?}", e)])
    }
}
