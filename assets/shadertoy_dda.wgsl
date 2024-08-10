#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput


struct ShadertoyInputs {
    iResolution: vec2<f32>,
    iTime: f32,
    iMouse: vec4<f32>,
    _webgl2_padding: f32,
}
@group(0) @binding(0) var<uniform> inputs: ShadertoyInputs;



fn sdSphere(p: vec3<f32>, d: f32) -> f32 {
    return length(p) - d;
}

fn sdBox(p: vec3<f32>, b: f32) -> f32 {
  let d = abs(p) - b;
  return min(max(d.x,max(d.y,d.z)), 0.0) +
         length(max(d,vec3<f32>(0.0)));
}
	
fn getVoxel(c: vec3<i32>) -> bool {
	let p = vec3<f32>(c) + 0.5;
	let d = min(
        max(-sdSphere(p, 7.5), sdBox(p, 6.0)),
        -sdSphere(p, 100.0)
    );
	return d < 0.0;
}

fn rotate2d(v: vec2<f32>, a: f32) -> vec2<f32> {
	let sinA = sin(a);
	let cosA = cos(a);
	return vec2<f32>(v.x * cosA - v.y * sinA, v.y * cosA + v.x * sinA);	
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
	let screenPos = in.uv * 2.0 - 1.0;
	let cameraDir = vec3<f32>(0.0, 0.0, 0.8);
	let cameraPlaneU = vec3<f32>(1.0, 0.0, 0.0);
	let cameraPlaneV = vec3<f32>(0.0, 1.0, 0.0);
	var rayDir = cameraDir + screenPos.x * cameraPlaneU + screenPos.y * cameraPlaneV;
	var rayPos = vec3<f32>(0.0, 2.0 * sin(inputs.iTime * 2.7), -12.0);

    let _1 = rotate2d(rayPos.xz, inputs.iTime);
	rayPos = vec3<f32>(_1.x, rayPos.y, _1.y);
    let _2 = rotate2d(rayDir.xz, inputs.iTime);
	rayDir = vec3<f32>(_2.x, rayDir.y, _2.y);
	
    let deltaDist = abs(1.0 / rayDir);
	let stepDir = vec3<i32>(sign(rayDir));
	var cell = vec3<i32>(floor(rayPos));
	var sideDist = (sign(rayDir) * (vec3<f32>(cell) - rayPos) + (sign(rayDir) * 0.5) + 0.5) * deltaDist; 
    var side = 0u;
	
	for (var i = 0; i < 256; i++) {
		if getVoxel(cell) { break; }

        if (sideDist.x < sideDist.y) {
            if (sideDist.x < sideDist.z) {
                sideDist.x += deltaDist.x;
                cell.x += stepDir.x;
                side = 0u;
            }
            else {
                sideDist.z += deltaDist.z;
                cell.z += stepDir.z;
                side = 2u;
            }
        }
        else {
            if (sideDist.y < sideDist.z) {
                sideDist.y += deltaDist.y;
                cell.y += stepDir.y;
                side = 1u;
            }
            else {
                sideDist.z += deltaDist.z;
                cell.z += stepDir.z;
                side = 2u;
            }
        }

	}
	
	let color = vec3<f32>(f32(side) * 0.25 + 0.5);
    return vec4<f32>(color, 1.0);
}