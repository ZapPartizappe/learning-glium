use std::{fs, io::{Cursor, Read}};

use image::ImageFormat;
use winit::{
	event::{
		Event,
		WindowEvent
	},
	event_loop::EventLoopBuilder
};
use glium::{
	backend::glutin::SimpleWindowBuilder, implement_vertex, index::{
		NoIndices,
		PrimitiveType
	}, texture::RawImage2d, uniform, Program, Surface, Texture2d, VertexBuffer
};

#[derive(Clone, Copy)]
struct Vertex {
	position: [f32; 2],
	tex_coords: [f32; 2],
}

fn main()
{
	implement_vertex!(Vertex, position, tex_coords);

	let event_loop = EventLoopBuilder::new().build().expect("Event loop building...");

	let (window, display) = SimpleWindowBuilder::new().with_title("Wow").build(&event_loop);

	let shape = vec![
		Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] },
		Vertex { position: [ 0.5, -0.5], tex_coords: [1.0, 0.0] },
		Vertex { position: [ 0.5,  0.5], tex_coords: [1.0, 1.0] },

		Vertex { position: [ 0.5,  0.5], tex_coords: [1.0, 1.0] },
		Vertex { position: [-0.5,  0.5], tex_coords: [0.0, 1.0] },
		Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] }
	];

	let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();
	let indices = NoIndices(PrimitiveType::TrianglesList);

	let image = image::load(
		Cursor::new(&include_bytes!("../images/textures/basil.png")),
		ImageFormat::Png
	).unwrap().to_rgba8();
	let image_dimensions = image.dimensions();
	let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

	let texture = Texture2d::new(&display, image).unwrap();

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

					let uniforms = uniform! {
						matrix: [
							[t_cos, t_sin, 0.0, 0.0],
							[-t_sin, t_cos, 0.0, 0.0],
							[0.0, 0.0, 1.0, 0.0],
							[0.0, 0.0, 0.0, 1.0],
						],
						tex: &texture
					};

					let mut target = display.draw();

					target.clear_color(0.0, 0.0, 0.0, 1.0);
					target.draw(
						&vertex_buffer, &indices, &program, &uniforms,
						&Default::default()
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
