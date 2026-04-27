use askama::Template;
use eternize_models::album::Album;

#[derive(Template, Debug, Clone)]
#[template(path = "sections/album.html")]
pub struct AlbumSection {
    pub model: Album,
}
