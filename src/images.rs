use rocket::get;
use rocket::response::NamedFile;
use std::path::Path;

/// Gets the image with the specified ID
///
/// If there is no image with the given ID, this function returns a 404
#[get("/<image_file_name>")]
pub fn get_image(image_file_name: String) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(image_file_name)).ok()
}
