use askama::Template;

mod pages;
mod sections;

pub fn render_customize_page() -> String {
    let page = pages::customize::CustomizePage {
        page_meta_title: "Bruno C.".to_string(),
        user_ordered_sections: vec![
            pages::customize::SectionType::Hero,
            pages::customize::SectionType::Vault,
        ],
    };

    page.render().unwrap()
}
