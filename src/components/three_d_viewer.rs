use dioxus::prelude::*;
// use dioxus_web::WebEventExt;
// use web_sys::HtmlCanvasElement;

// use three_d::{
//     Camera, CpuMesh, FrameOutput, Gm, Mesh, PhysicalMaterial,
//     Window, Srgba, Mat4, vec3, degrees
// };
// // use web_sys::HtmlCanvasElement;

#[component]
pub fn ThreeDViewer() -> Element {
    // let canvas_ref = use_signal::<Option<HtmlCanvasElement>>(|| None);

    // use_effect(move || {
    //     if let Some(canvas) = canvas_ref.read().clone() {
    //         // Call the `three-d` setup function with the canvas element.
    //         setup_three_d_scene(canvas);
    //     }
    // });

    rsx! {
        div {
            class: "w-full h-full",
            canvas {
                id: "three-d-canvas",
                class: "w-full h-full",
                // Correct syntax to get the typed canvas element
                // onmounted: move |event| {
                //     if let Some(canvas) = event.get_dom_element::<HtmlCanvasElement>() {
                //         canvas_ref.set(Some(canvas));
                //     }
                // }
            }
        }
    }
}
// fn setup_three_d_scene(canvas: HtmlCanvasElement) {
//     // This is the main function that runs the 3D rendering loop.
//     let window = Window::from_canvas(canvas).unwrap();
//     let gl = window.gl();
//     let viewport = window.viewport();

//     let mut camera = Camera::new_perspective(
//         &gl,
//         vec3(0.0, 2.0, 4.0),
//         vec3(0.0, 0.0, 0.0),
//         vec3(0.0, 1.0, 0.0),
//         degrees(45.0),
//         viewport.aspect(),
//         0.1,
//         1000.0,
//     );

//     let mut renderer = ForwardRenderer::new(&gl).unwrap();
//     let light = three_d::AmbientLight::new(&gl, 0.4, Srgba::WHITE);

//     // Create a sphere to display
//     let mut model = Gm::new(
//         Mesh::new(&gl, &CpuMesh::sphere(16)).unwrap(),
//         PhysicalMaterial::new(&gl, &three_d::CpuMaterial::default()).unwrap(),
//     );

//     window.render_loop(move |frame_input| {
//         // Adjust the camera aspect ratio if the window is resized
//         camera.set_aspect(frame_input.viewport.aspect());
        
//         // Rotate the sphere
//         model.animate(frame_input.time);

//         // Clear the screen and render the scene
//         renderer.render(
//             &frame_input.viewport,
//             &[model.as_ref()],
//             &[&light, &camera],
//         );

//         FrameOutput::default()
//     });
// }