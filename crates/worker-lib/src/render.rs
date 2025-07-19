use std::io::Cursor;

use protocol::Rendered;
use typst::layout::{Axis, PagedDocument, Size};

use crate::diagnostic::format_diagnostics;
use crate::sandbox::Sandbox;

const DESIRED_RESOLUTION: f32 = 3000.0;
const MAX_SIZE: f32 = 30000.0;
const MAX_PIXELS_PER_POINT: f32 = 30.0;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
	Png,
	Pdf,
}

impl Default for OutputFormat {
	fn default() -> Self {
		Self::Png
	}
}

#[derive(Debug, thiserror::Error)]
#[error(
	"rendered output was too big: the {axis:?} axis was {size} pt but the maximum is {MAX_SIZE}"
)]
pub struct TooBig {
	size: f32,
	axis: Axis,
}

fn determine_pixels_per_point(size: Size, desired_resolution: f32) -> Result<f32, TooBig> {
	// We want to truncate.
	#![allow(clippy::cast_possible_truncation)]

	let x = size.x.to_pt() as f32;
	let y = size.y.to_pt() as f32;

	if x > MAX_SIZE {
		Err(TooBig {
			size: x,
			axis: Axis::X,
		})
	} else if y > MAX_SIZE {
		Err(TooBig {
			size: y,
			axis: Axis::Y,
		})
	} else {
		let area = x * y;
		let nominal = desired_resolution / area.sqrt();
		Ok(nominal.min(MAX_PIXELS_PER_POINT))
	}
}

fn to_string(v: impl ToString) -> String {
	v.to_string()
}

const PAGE_LIMIT: usize = 5;
const BYTES_LIMIT: usize = 25 * 1024 * 1024;

pub fn render(sandbox: &Sandbox, source: String) -> Result<Rendered, String> {
	render_with_resolution(sandbox, source, DESIRED_RESOLUTION)
}

pub fn render_with_resolution(sandbox: &Sandbox, source: String, resolution: f32) -> Result<Rendered, String> {
	render_with_format(sandbox, source, OutputFormat::Png, Some(resolution))
}

pub fn render_with_format(sandbox: &Sandbox, source: String, format: OutputFormat, resolution: Option<f32>) -> Result<Rendered, String> {
	let world = sandbox.with_source(source);

	let document = typst::compile::<PagedDocument>(&world);
	let warnings = document.warnings;
	let document = document
		.output
		.map_err(|diags| format_diagnostics(&world, &diags))?;

	let mut total_attachment_size = 0;

	let output_data = match format {
		OutputFormat::Png => {
			// For PNG, use resolution parameter (default to DESIRED_RESOLUTION if not provided)
			let res = resolution.unwrap_or(DESIRED_RESOLUTION);
			
			document
				.pages
				.iter()
				.take(PAGE_LIMIT)
				.map(|page| {
					let pixels_per_point = determine_pixels_per_point(page.frame.size(), res).map_err(to_string)?;
					let pixmap = typst_render::render(page, pixels_per_point);

					let mut writer = Cursor::new(Vec::new());

					// The unwrap will never fail since `Vec`'s `Write` implementation is infallible.
					image::write_buffer_with_format(
						&mut writer,
						bytemuck::cast_slice(pixmap.pixels()),
						pixmap.width(),
						pixmap.height(),
						image::ColorType::Rgba8,
						image::ImageFormat::Png,
					)
					.unwrap();

					Ok(writer.into_inner())
				})
				.take_while(|image| {
					if let Ok(image) = image {
						total_attachment_size += image.len();
						total_attachment_size <= BYTES_LIMIT
					} else {
						true
					}
				})
				.collect::<Result<Vec<_>, String>>()?
		}
		OutputFormat::Pdf => {
			// For PDF, resolution is ignored since it's vector-based
			// Generate a single PDF containing all pages
			let pdf_options = typst_pdf::PdfOptions::default();
			let pdf_data = typst_pdf::pdf(&document, &pdf_options).map_err(|diags| format_diagnostics(&world, &diags))?;
			vec![pdf_data]
		}
	};

	let more_pages = document.pages.len() - output_data.len();

	Ok(Rendered {
		images: output_data,
		more_pages,
		warnings: format_diagnostics(&world, &warnings),
	})
}
