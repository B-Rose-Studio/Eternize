use askama::Template;
use eternize_models::{customize_page::CustomizePage, section::Section};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};
use strum_macros::EnumString;

#[derive(Serialize, Deserialize, Debug, Clone, EnumString)]
pub enum SectionType {
    #[strum(ascii_case_insensitive)]
    Hero,
    #[strum(ascii_case_insensitive)]
    Album,
    #[strum(ascii_case_insensitive)]
    Timeline,
    Glass,
    #[strum(ascii_case_insensitive)]
    Gift,
}

#[derive(Template, Debug, Clone)]
#[template(path = "index.html")]
pub struct CustomizePageTemplate<'a> {
    pub page_meta_title: &'a str,
    pub user_ordered_sections: Vec<SectionType>,
}

impl<'a> CustomizePageTemplate<'a> {
    pub fn render(
        params: CustomizePage,
        mut sections: Vec<(Section, HashMap<String, String>)>,
    ) -> String {
        sections.sort_by(|a, b| a.0.order.cmp(&b.0.order));

        let sections_types: Vec<SectionType> = sections
            .iter()
            .map(|section| SectionType::from_str(&section.0.name).unwrap())
            .collect();

        let page = CustomizePageTemplate {
            page_meta_title: &params.title,
            user_ordered_sections: sections_types,
        };

        let mut html = page.render().unwrap();

        for section in sections {
            match SectionType::from_str(&section.0.name).unwrap() {
                SectionType::Hero => {
                    html = html.replacen("<Section></Section>", "Hero", 1);
                }

                SectionType::Album => {
                    html = html.replacen("<Section></Section>", "Album", 1);
                }

                SectionType::Timeline => {
                    html = html.replacen("<Section></Section>", "Timeline", 1);
                }

                SectionType::Glass => {
                    html = html.replacen("<Section></Section>", "Glass", 1);
                }

                SectionType::Gift => {
                    html = html.replacen("<Section></Section>", "Gift", 1);
                }
            }
        }

        html
    }

    pub fn test() -> String {
        let sections = vec![SectionType::Hero, SectionType::Glass, SectionType::Gift];

        let page = CustomizePageTemplate {
            page_meta_title: "Teste",
            user_ordered_sections: sections.clone(),
        };

        let mut html = page.render().unwrap();

        for section in sections {
            match section {
                SectionType::Hero => {
                    html = html.replacen("<Section></Section>", "Hero", 1);
                }

                SectionType::Glass => {
                    html = html.replacen("<Section></Section>", "Glass", 1);
                }

                SectionType::Gift => {
                    html = html.replacen("<Section></Section>", "Gift", 1);
                }

                _ => {}
            }
        }

        html
    }
}
