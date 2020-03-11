use rocket::get;

/// Gets the image with the specified ID
///
/// If there is no image with the given ID, this function returns a 404
#[get("/<image_id>")]
pub fn get_image(image_id: String) -> String {
    "hihi".into()
}
