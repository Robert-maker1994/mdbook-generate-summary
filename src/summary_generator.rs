use mdbook::book::{Book};
use mdbook::errors::Result;
use mdbook::preprocess::Preprocessor;
use mdbook::preprocess::PreprocessorContext;
 
pub struct SummaryGenerator;
const SUMMARY_NAME: &str = "summary-generator";

impl Preprocessor for SummaryGenerator {
    fn name(&self) -> &str {
        SUMMARY_NAME
    }

    fn run(&self, _: &PreprocessorContext, book: Book) -> Result<Book> {
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}
