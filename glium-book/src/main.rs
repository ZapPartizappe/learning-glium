use std::{
	f32::consts::PI,
	fs,
	io::{Cursor, Read}
};

use image::ImageFormat;
use winit::{
	event::{
		Event,
		WindowEvent
	},
	event_loop::EventLoopBuilder
};

use glium::{
	backend::glutin::SimpleWindowBuilder, implement_vertex, index::{NoIndices, PrimitiveType}, texture::RawImage2d, uniform, BackfaceCullingMode, Depth, DrawParameters, Program, Surface, Texture2d, VertexBuffer
};

#[derive(Clone, Copy)]
struct Vertex {
	position: [f32; 3],
	normal: [f32; 3],
	tex_coords: [f32; 2],
}

mod teapot;
mod support;

fn main()
{
	implement_vertex!(Vertex, position, normal, tex_coords);

	let event_loop = EventLoopBuilder::new().build().expect("Event loop building...");

	let (window, display) = SimpleWindowBuilder::new().with_title("Wow").build(&event_loop);

	let shape = VertexBuffer::new(&display, &[
		Vertex { position: [-1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 1.0] },
		Vertex { position: [ 1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 1.0] },
		Vertex { position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
		Vertex { position: [ 1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 0.0] },
	]).unwrap();

	let image = image::load(
		Cursor::new(&include_bytes!("../images/textures/diffuse.jpg")),
		ImageFormat::Jpeg
	).unwrap().to_rgba8();
	let image_dimensions = image.dimensions();
	let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
	let diffuse_tex = Texture2d::new(&display, image).unwrap();

	let image = image::load(
		Cursor::new(&include_bytes!("../images/textures/normal.png")),
		ImageFormat::Png
	).unwrap().to_rgba8();
	let image_dimensions = image.dimensions();
	let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
	let normal_tex = Texture2d::new(&display, image).unwrap();

	let mut vert_shader_src = String::new();
	let mut frag_shader_src = String::new();

	fs::File::open("shaders/vert_shader.vert").unwrap().read_to_string(&mut vert_shader_src).unwrap();
	fs::File::open("shaders/frag_shader.frag").unwrap().read_to_string(&mut frag_shader_src).unwrap();

	let program =
		Program::from_source(
			&display,
			&vert_shader_src,
			&frag_shader_src,
			None
		).unwrap();

	let draw_parameters = DrawParameters {
		depth: Depth {
			test: glium::DepthTest::IfLess,
			write: true,
			.. Default::default()
		},
		.. Default::default()
	};

	let mut t: f32 = 0.0;

	let _ = event_loop.run(move |event, window_target| {
		match event {
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::CloseRequested => window_target.exit(),
				WindowEvent::Resized(window_size) => {
					display.resize(window_size.into());
				},
				WindowEvent::RedrawRequested => {
					t += 0.02;

					let (t_sin, t_cos) = t.sin_cos();

					let mut target = display.draw();

					target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

					let model = [
						[t_cos,	 t_sin,	0.0, 0.0],
						[-t_sin, t_cos,	0.0, 0.0],
						[0.0,	 0.0,	1.0, 0.0],
						[0.0,	 0.0,	0.0, 1.0f32]
					];
					let light = [1.4, 0.4, 0.7f32];
					let perspective = support::perspective_mat(target.get_dimensions(), PI / 3.0);
					let view =
						support::view_mat(&[0.5, 0.2, -3.0], &[-0.5, -0.2, 3.0], &[0.0, 1.0, 0.0]);

					let uniforms =
						uniform! {
							model: model,
							view: view,
							perspective: perspective,
							u_light: light,
							diffuse_tex: &diffuse_tex,
							normal_tex: &normal_tex,
						};

					target.draw(
						&shape, NoIndices(PrimitiveType::TriangleStrip), &program, &uniforms, &draw_parameters
					).unwrap();
					target.finish().unwrap();
				},
				_ => ()
			},
			Event::AboutToWait => {
				window.request_redraw();
			},
			_ => (),
		}
	});
}

#[allow(dead_code)]
fn spinny_basil()
{
	
}