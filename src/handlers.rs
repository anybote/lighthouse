use crate::config::{ContentConfig, ImageConfig, ImageMetadatum};
use askama_axum::Template;
use axum::extract::{Path, State};
use std::{cmp, sync::Arc};

const IMAGE_PAGE_SIZE: usize = 3;

/// Used to hydrate the home page
#[derive(Template)]
#[template(path = "home.html.jinja")]
pub struct HomeTemplate {
    title: String,
    header: String,
    subheader: String,
    // a subset of image metadata loaded when the homepage is visited
    image_metadata: Vec<ImageMetadatum>,
}

/// Config needed to render the home page
pub struct HomeState {
    title: String,
    header: String,
    subheader: String,
    // all image metadata available to the page
    image_metadata: Vec<ImageMetadatum>,
}

impl HomeState {
    pub fn from(content_config: &ContentConfig, image_config: &ImageConfig) -> HomeState {
        HomeState {
            title: content_config.title.clone(),
            header: content_config.header.clone(),
            subheader: content_config.subheader.clone(),
            image_metadata: image_config.images.clone(),
        }
    }
}

/// Route handler for the home page
pub async fn home(State(home_state): State<Arc<HomeState>>) -> HomeTemplate {
    let (page_start, page_end) =
        calculate_page_bounds(0, IMAGE_PAGE_SIZE, home_state.image_metadata.len());

    HomeTemplate {
        title: home_state.title.clone(),
        header: home_state.header.clone(),
        subheader: home_state.subheader.clone(),
        image_metadata: Vec::from(&home_state.image_metadata[page_start..page_end]),
    }
}

/// Used to hydrate each subsequent page that's loaded as the user scrolls
#[derive(Template)]
#[template(path = "tile.html.jinja")]
pub struct TileTemplate {
    page_index: usize,
    // a subset of image metadata to be loaded as the user scrolls
    image_metadata: Vec<ImageMetadatum>,
    is_final_page: bool,
}

/// Config needed to render images
pub struct TileState {
    // all image metadata available to the page
    image_metadata: Vec<ImageMetadatum>,
}

impl TileState {
    pub fn from(image_config: &ImageConfig) -> TileState {
        TileState {
            image_metadata: image_config.images.clone(),
        }
    }
}

pub async fn tile(
    State(tile_state): State<Arc<TileState>>,
    Path(page_index): Path<usize>,
) -> TileTemplate {
    let (page_start, page_end) =
        calculate_page_bounds(page_index, IMAGE_PAGE_SIZE, tile_state.image_metadata.len());

    TileTemplate {
        page_index,
        image_metadata: Vec::from(&tile_state.image_metadata[page_start..page_end]),
        is_final_page: page_end == tile_state.image_metadata.len(),
    }
}

/// Calculate the start (inclusive) and end (exclusive) bounds of the page. The page index starts at 0.
fn calculate_page_bounds(
    page_index: usize,
    page_size: usize,
    num_objects: usize,
) -> (usize, usize) {
    if page_size == 0 {
        panic!("page size must be greater than zero")
    }

    let num_pages = cmp::max((num_objects + page_size - 1) / page_size, 1);
    if page_index >= num_pages {
        panic!("page {page_index} is greater than the number of pages {num_pages}, cannot calculate page bounds");
    }
    if page_index == num_pages - 1 {
        return (page_index * page_size, num_objects);
    }
    (page_index * page_size, (page_index + 1) * page_size)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_calculate_page_bounds() {
        // page size is zero
        let result = panic::catch_unwind(|| calculate_page_bounds(0, 0, 28));
        assert!(result.is_err());

        let result = calculate_page_bounds(0, 5, 28);
        assert_eq!(result, (0, 5));

        let result = calculate_page_bounds(1, 5, 28);
        assert_eq!(result, (5, 10));

        let result = calculate_page_bounds(2, 5, 28);
        assert_eq!(result, (10, 15));

        let result = calculate_page_bounds(3, 5, 28);
        assert_eq!(result, (15, 20));

        let result = calculate_page_bounds(4, 5, 28);
        assert_eq!(result, (20, 25));

        let result = calculate_page_bounds(5, 5, 28);
        assert_eq!(result, (25, 28));

        // page index is out of bounds
        let result = panic::catch_unwind(|| calculate_page_bounds(6, 5, 28));
        assert!(result.is_err());

        // no objects to paginate
        let result = calculate_page_bounds(0, 5, 0);
        assert_eq!(result, (0, 0));
    }
}
