use winit::{
	event::{
		Event,
		WindowEvent
	},
	event_loop::EventLoopBuilder
};
use glium::{
	backend::glutin::SimpleWindowBuilder,
	Surface
};

fn main()
{
	let event_loop = EventLoopBuilder::new().build().expect("Event loop building...");
	let (_window, display) = SimpleWindowBuilder::new().build(&event_loop);

	let mut target = display.draw();

	target.clear_color(0.0, 0.0, 1.0, 1.0);
	target.finish().unwrap();

	let _ = event_loop.run(|event, window_target| {
		match event {
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::CloseRequested => window_target.exit(),
				_ => ()
			},
			_ => (),
		}
	});
}
