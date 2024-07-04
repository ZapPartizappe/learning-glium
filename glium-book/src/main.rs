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
	}, uniform, uniforms::EmptyUniforms, Program, Surface, VertexBuffer
};

#[derive(Clone, Copy)]
struct Vertex {
	position: [f32; 2],
}

fn main()
{
	implement_vertex!(Vertex, position);

	let event_loop = EventLoopBuilder::new().build().expect("Event loop building...");

	let (window, display) = SimpleWindowBuilder::new().build(&event_loop);

	let shape = vec![
		Vertex { position: [-0.5, -0.5] },
		Vertex { position: [ 0.0,  0.5] },
		Vertex { position: [ 0.5, -0.25] }
	];

	let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();
	let indices = NoIndices(PrimitiveType::TrianglesList);

	let vert_shader_src = r#"
		#version 140

		in vec2 position;

		uniform mat4 matrix;

		void main()
		{
			gl_Position = matrix * vec4(position, 0.0, 1.0);
		}
	"#;

	let frag_shader_src = r#"
		#version 140

		out vec4 color;

		void main()
		{
			color = vec4(1.0, 0.0, 0.0, 1.0);
		}
	"#;

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

					let t_cos = t.cos();
					let t_sin = t.sin();

					let uniforms = uniform! {
						matrix: [
							[t_cos, t_sin, 0.0, 0.0],
							[-t_sin, t_cos, 0.0, 0.0],
							[0.0, 0.0, 1.0, 0.0],
							[0.0, 0.0, 0.0, 1.0],
						]
					};

					let mut target = display.draw();

					target.clear_color(0.0, 0.0, 1.0, 1.0);
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
