use crate::sections::{album::Album, gift::Gift, glass::Glass, hero::Hero, timeline::Timeline};
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
#[template(path = "customize/index.html")]
pub struct CustomizePageTemplate<'a> {
    page_meta_title: &'a str,
    user_ordered_sections: Vec<SectionType>,

    hero: Option<Hero>,
    album: Option<Album>,
    glass: Option<Glass>,
    gift: Option<Gift>,
    timeline: Option<Timeline>,
}

impl<'a> CustomizePageTemplate<'a> {
    fn new(
        title: &'a str,
        sections: Vec<SectionType>,
        propertys: &HashMap<String, String>,
    ) -> Self {
        let mut object = Self {
            page_meta_title: title,
            user_ordered_sections: sections.clone(),

            album: None,
            gift: None,
            glass: None,
            hero: None,
            timeline: None,
        };

        for section in sections {
            match section {
                SectionType::Hero => {
                    object.hero = propertys.try_into().ok();
                }

                SectionType::Glass => {
                    object.glass = propertys.try_into().ok();
                }

                SectionType::Timeline => {
                    object.timeline = propertys.try_into().ok();
                }

                SectionType::Album => {
                    object.album = propertys.try_into().ok();
                }

                SectionType::Gift => {
                    object.gift = propertys.try_into().ok();
                }
            }
        }

        object
    }

    pub fn render(
        params: CustomizePage,
        mut sections: Vec<Section>,
        propertys: HashMap<String, String>,
    ) -> String {
        sections.sort_by(|a, b| a.order.cmp(&b.order));

        let sections_types: Vec<SectionType> = sections
            .iter()
            .map(|section| SectionType::from_str(&section.name).unwrap())
            .collect();

        let page = CustomizePageTemplate::new(&params.title, sections_types, &propertys);
        page.render().unwrap()
    }
}
