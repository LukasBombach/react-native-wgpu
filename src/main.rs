use deno_core::*;
use serde::Deserialize;
use std::borrow::Cow;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Triangle {
    size: f32,
    position: (f32, f32),
    color: [f32; 4],
}

#[derive(Default)]
struct AppState {
    triangles: Vec<Triangle>,
}

#[derive(Debug, Deserialize)]
struct TriangleInput {
    size: f32,
    position: [f32; 2],
    color: [f32; 4],
}

/// Der Op, der ein Dreieck erstellt. Die Eingabe wird per Serde
/// automatisch aus dem übergebenen JSON deserialisiert.
#[op2]
fn op_create_triangle(
    state: &mut OpState,
    #[serde] input: TriangleInput,
) -> Result<(), deno_error::JsErrorBox> {
    // Hole den globalen Zustand aus dem OpState.
    let app_state = state.borrow::<Arc<Mutex<AppState>>>().clone();
    {
        let mut app_state = app_state.lock().unwrap();
        app_state.triangles.push(Triangle {
            size: input.size,
            position: (input.position[0], input.position[1]),
            color: input.color,
        });
    }
    Ok(())
}

fn main() {
    // Initialisiere den globalen Zustand.
    let app_state = Arc::new(Mutex::new(AppState::default()));

    // Erstelle die Extension, indem du ein OpDecl mittels des Aufrufs der Funktion erhältst.
    const DECL: OpDecl = op_create_triangle();
    let ext = Extension {
        name: "triangle_ext",
        ops: Cow::Borrowed(&[DECL]),
        ..Default::default()
    };

    // Initialisiere die JavaScript-Laufzeit mit der Extension.
    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![ext],
        ..Default::default()
    });

    // In den OpState den globalen Zustand einfügen.
    runtime.op_state().borrow_mut().put(app_state);

    // Führe JavaScript-Code aus, der den Op aufruft.
    runtime
        .execute_script(
            "<init>",
            r#"
        class Triangle {
            constructor(size, position, color) {
                // Ruft den Rust-Op auf, um ein Dreieck zu erstellen.
                Deno.core.ops.op_create_triangle({ size, position, color });
            }
        }
        // Erstelle zwei Dreiecke:
        new Triangle(100, [150, 150], [1.0, 0.0, 0.0, 1.0]);
        new Triangle(80, [300, 200], [0.0, 1.0, 0.0, 1.0]);
        Deno.core.print("Triangles created\n");
        "#,
        )
        .unwrap();

    // Simulierter "Rendering-Loop", der die erzeugten Dreiecke ausgibt.
    loop {
        {
            let op_state = runtime.op_state();
            let state = op_state.borrow();
            let app_state = state.borrow::<Arc<Mutex<AppState>>>().clone();
            let app_state = app_state.lock().unwrap();
            for triangle in &app_state.triangles {
                println!(
                    "Render Triangle: size: {}, position: {:?}, color: {:?}",
                    triangle.size, triangle.position, triangle.color
                );
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
