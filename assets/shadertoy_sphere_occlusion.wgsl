#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

struct ShadertoyInputs {
    iResolution: vec2<f32>,
    iTime: f32,
    iMouse: vec4<f32>,
    _webgl2_padding: f32,
}
@group(0) @binding(0) var<uniform> uni: ShadertoyInputs;


fn sphIntersect(ro: vec3<f32>, rd: vec3<f32>, sph: vec4<f32>) -> f32 {
	let oc: vec3<f32> = ro - sph.xyz;
	let b: f32 = dot(oc, rd);
	let c: f32 = dot(oc, oc) - sph.w * sph.w;
	var h: f32 = b * b - c;
	if (h < 0.) {	return -1.;
 }
	return -b - sqrt(h);
} 

fn sphOcclusion(pos: vec3<f32>, nor: vec3<f32>, sph: vec4<f32>) -> f32 {
	let di: vec3<f32> = sph.xyz - pos;
	let l: f32 = length(di);
	let nl: f32 = dot(nor, di / l);
	let h: f32 = l / sph.w;
	let h2: f32 = h * h;
	let k2: f32 = 1. - h2 * nl * nl;
	var res: f32 = max(0., nl) / h2;
	if (k2 > 0.001) {
		res = nl * acos(-nl * sqrt((h2 - 1.) / (1. - nl * nl))) - sqrt(k2 * (h2 - 1.));
		res = res / h2 + atan(sqrt(k2 / (h2 - 1.)));
		res = res / (3.141593);
	}
	return res;
} 

fn iPlane(ro: vec3<f32>, rd: vec3<f32>) -> f32 {
	return (-1. - ro.y) / rd.y;
} 

fn hash21(p: f32) -> vec2<f32> {
	var p3: vec3<f32> = fract(vec3<f32>(p) * vec3<f32>(0.1031, 0.103, 0.0973));
	p3 = p3 + (dot(p3, p3.yzx + 33.33));
	return fract((p3.xx + p3.yz) * p3.zy);
} 

fn hash13(p3: vec3<f32>) -> f32 {
	var p3_var = p3;
	p3_var = fract(p3_var * 0.1031);
	p3_var = p3_var + (dot(p3_var, p3_var.zyx + 31.32));
	return fract((p3_var.x + p3_var.y) * p3_var.z);
} 



@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
	var p: vec2<f32> = (2. * in.uv - 1.0) * uni.iResolution.x / uni.iResolution.y;
    p.y *= -1.;

	var s: f32 = (2. * uni.iMouse.x - uni.iResolution.x) / uni.iResolution.y;
	if (uni.iMouse.z < 0.001) { s = 0.; }

	let ro: vec3<f32> = vec3<f32>(0., 0., 4.);
	let rd: vec3<f32> = normalize(vec3<f32>(p, -2.));
	let sph: vec4<f32> = vec4<f32>(cos(uni.iTime + vec3<f32>(2., 1., 1.) + 0.) * vec3<f32>(1.5, 1.2, 1.), 1.);
	let rrr: f32 = hash13(vec3<f32>(uni.iTime, p * 100.));
	var col: vec3<f32> = vec3<f32>(0.);
	var tmin: f32 = 10000000000.;
	let t1: f32 = iPlane(ro, rd);
	if (t1 > 0.) {
		tmin = t1;
		var pos: vec3<f32> = ro + tmin * rd;
		var nor: vec3<f32> = vec3<f32>(0., 1., 0.);
		var occ: f32 = 0.;
		if (p.x > s) {
			occ = sphOcclusion(pos, nor, sph);
		} else { 
			let ru: vec3<f32> = normalize(cross(nor, vec3<f32>(0., 1., 1.)));
			let rv: vec3<f32> = normalize(cross(ru, nor));
			occ = 0.;

			for (var i: i32 = 0; i < 256; i = i + 1) {
				let aa: vec2<f32> = hash21(rrr + f32(i) * 203.1);
				let ra: f32 = sqrt(aa.y);
				let rx: f32 = ra * cos(6.2831 * aa.x);
				let ry: f32 = ra * sin(6.2831 * aa.x);
				let rz: f32 = sqrt(1. - aa.y);
				let dir: vec3<f32> = vec3<f32>(rx * ru + ry * rv + rz * nor);
				let res: f32 = sphIntersect(pos, dir, sph);
				occ = occ + (step(0., res));
			}

			occ = occ / (256.);
		}
		col = vec3<f32>(1.);
		col = col * (1. - occ);
	}
	let t2: f32 = sphIntersect(ro, rd, sph);
	if (t2 > 0. && t2 < tmin) {
		tmin = t2;
		let t: f32 = t2;
		let pos: vec3<f32> = ro + t * rd;
		let nor: vec3<f32> = normalize(pos - sph.xyz);
		col = vec3<f32>(1.2);
		col = col * (0.6 + 0.4 * nor.y);
	}
	col = col * (exp(-0.05 * tmin));
	let e: f32 = 2. / uni.iResolution.y;
	col = col * (smoothstep(0., 2. * e, abs(p.x - s)));
	return vec4<f32>(col, 1.);
} 

