use std::{
	f32::consts::PI, fs, io::Read
};

use winit::{
	event::{
		Event,
		WindowEvent
	},
	event_loop::EventLoopBuilder
};

use glium::{
	backend::glutin::SimpleWindowBuilder, implement_vertex, index::PrimitiveType, uniform, BackfaceCullingMode, Depth, DrawParameters, IndexBuffer, Program, Surface, VertexBuffer
};

#[derive(Clone, Copy)]
struct Vertex {
	position: [f32; 2],
	tex_coords: [f32; 2],
}

mod teapot;
mod support;

fn main()
{
	implement_vertex!(Vertex, position, tex_coords);

	let event_loop = EventLoopBuilder::new().build().expect("Event loop building...");

	let (window, display) = SimpleWindowBuilder::new().with_title("Wow").build(&event_loop);

	let positions = VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
	let normals = VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
	let indices =
		IndexBuffer::new(&display, PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();

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
		backface_culling: BackfaceCullingMode::CullClockwise,
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

					let t_sin = 0.01 * t.sin();
					let t_cos = 0.01 * t.cos();

					let mut target = display.draw();

					target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

					let position = [2.0, 1.0, 1.0];

					let model = [
						[t_cos, 0.0, -t_sin, 0.0],
						[0.0, 0.01, 0.0, 0.0],
						[t_sin, 0.0, t_cos, 0.0],
						[0.0, 0.0, 2.0, 1.0f32]
					];
					let light = [1.4, 0.4, -0.7f32];
					let perspective = support::perspective_mat(target.get_dimensions(), PI / 3.0);
					let view =
						support::view_mat(&position, &[-2.0, -1.0, 1.0], &[0.0, 1.0, 0.0]);

					let uniforms =
						uniform! {
							model: model,
							view: view,
							perspective: perspective,
							u_light: light,
						};

					target.draw(
						(&positions, &normals), &indices, &program, &uniforms, &draw_parameters
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