use crate::utils::uuid_from_str;
use eternize_models::customize_page::CustomizePage;
use eternize_models::section::Section;
use eternize_repository::PageRepository;
use eternize_repository::ReadMethod;
use futures::join;
use std::collections::HashMap;

pub struct PageServices {}

impl PageServices {
    pub async fn get_page_in_chain(
        page_repo: impl PageRepository,
        id: String,
    ) -> (Option<CustomizePage>, Vec<Section>, HashMap<String, String>) {
        let page_id = match uuid_from_str(&id) {
            Some(uuid) => uuid,
            None => return (None, vec![], HashMap::new()),
        };

        let future_page = page_repo.read(ReadMethod::ById(page_id.clone()));
        let future_sections = page_repo.get_sections(page_id.clone());
        let future_properties = page_repo.get_all_properties(page_id.clone());

        let (page_result, sections_result, properties_result) =
            join!(future_page, future_sections, future_properties);

        (
            page_result.map(|mut res| res.remove(0)).ok(),
            sections_result.unwrap_or_default(),
            properties_result.unwrap_or_default(),
        )
    }
}
