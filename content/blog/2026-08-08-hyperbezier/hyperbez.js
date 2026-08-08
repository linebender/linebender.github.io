"use strict";
/*
 * Hyperbezier tester.
 *
 * A hyperbezier is the curve traced with unit speed and tangent angle
 * theta(s), where curvature is a rational function of arc length s:
 *
 *     kappa(s) = (a*s + b) / q(s)^(3/2),   q(s) = c*s^2 + d*s + 1
 *
 * on s in [0,1] (then scaled to its chord). q is the denominator
 * quadratic: q constant gives circles and Euler spirals exactly; q
 * dipping toward 0 gives a corner. The raw form has five constants (two
 * linear, three quadratic), but scaling (q, a, b) -> (t*q, t^1.5*a,
 * t^1.5*b) leaves the curve unchanged -- a one-parameter gauge freedom,
 * so only four are effective. Setting q(0) = 1 above is one gauge
 * choice; solveMap uses another. theta(s) = integral of kappa has a
 * closed form; the position integral of e^(i*theta) does not
 * (Fresnel-like), so it is the one thing done numerically.
 *
 * The tester maps a cubic-Bezier-style control polygon -- endpoint
 * tangent angles th0, th1 and arm lengths l0, l1, in the chord frame --
 * to (a, b, c, d), and draws the result over the ghost cubic.
 */
/* Standard Gauss-Legendre nodes/weights (Newton on Legendre polynomials). */
/* ---------- Gauss-Legendre n=32 ---------- */
function gaussLegendre(n) {
  const x = new Float64Array(n), w = new Float64Array(n);
  for (let i = 0; i < n; i++) {
    let t = Math.cos(Math.PI*(i + 0.75)/(n + 0.5));
    for (let it = 0; it < 100; it++) {
      let p0 = 1, p1 = t;
      for (let k = 2; k <= n; k++) { const p2 = ((2*k-1)*t*p1 - (k-1)*p0)/k; p0 = p1; p1 = p2; }
      const dp = n*(t*p1 - p0)/(t*t - 1), dt = p1/dp;
      t -= dt; if (Math.abs(dt) < 1e-15) break;
    }
    let p0 = 1, p1 = t;
    for (let k = 2; k <= n; k++) { const p2 = ((2*k-1)*t*p1 - (k-1)*p0)/k; p0 = p1; p1 = p2; }
    const dp = n*(t*p1 - p0)/(t*t - 1);
    x[i] = t; w[i] = 2/((1 - t*t)*dp*dp);
  }
  return { x, w };
}
const GL = gaussLegendre(32);
const wrapA = a => { const T = 2*Math.PI; return ((a + Math.PI) % T + T) % T - Math.PI; };
const cAbs = z => Math.hypot(z[0], z[1]);

/* ---------- theta / integration machinery ---------- */
/*
 * Closed form of theta(t) = integral_0^t (a*s+b)/q^(3/2) ds, with q(0)=1:
 * theta = (A*t + B)/sqrt(q) + const. The second branch is a rearrangement
 * that avoids catastrophic cancellation when d^2 ~ 4c, i.e. the denominator
 * quadratic is a near-perfect square -- which includes the Euler/circle case q == 1.
 */
function makeTheta(a, b, c, d) {
  if (Math.abs(c) < 1e-12 && Math.abs(d) < 1e-12) return t => b*t + 0.5*a*t*t;
  const disc = d*d - 4*c, scale = Math.max(1, Math.abs(d*d), Math.abs(4*c));
  if (Math.abs(disc) > 0.02*scale) {
    const B = (4*a - 2*b*d)/disc, A = b + 0.5*B*d;
    return t => { const q = (c*t + d)*t + 1; return (A*t + B)/Math.sqrt(q) - B; };
  }
  return t => {
    const q = (c*t + d)*t + 1, sq = Math.sqrt(q), h = 2*c*t + d;
    return (t/sq)*(b + t*(4*a - 2*b*d)*(1 + (h*h + 4*c)/(d*h + 4*c*sq))/(4*(sq + 1)*(sq + 1)));
  };
}
/* Roots of kappa'(t): a quadratic. Used to place integration cuts. */
function kappaExtrema(a, b, c, d) {
  const qa = -4*a*c, qb = -(a*d + 6*b*c), qc = 2*a - 3*b*d;
  let roots = [];
  if (Math.abs(qa) < 1e-14) { if (Math.abs(qb) > 1e-14) roots = [-qc/qb]; }
  else {
    const disc = qb*qb - 4*qa*qc;
    if (disc >= 0) { const s = Math.sqrt(disc); roots = [(-qb - s)/(2*qa), (-qb + s)/(2*qa)]; }
  }
  return roots.filter(r => r > 0 && r < 1).sort((p, q) => p - q);
}
/*
 * Piecewise integration cuts: just the vertex of the denominator
 * quadratic, and only when it is a minimum (c > 0). Gauss-Legendre nodes
 * cluster quadratically at panel ends, so placing the q-minimum on a
 * panel boundary resolves the curvature spike there; for c < 0 the
 * vertex is a maximum and any dips sit at the interval ends, which GL
 * clustering already covers. With the map's clamps bounding how deep q
 * can dip, this is accurate to ~1e-4 of the chord vs dense reference.
 */
function cutPoints(a, b, c, d, t) {
  const cuts = [0];
  if (c > 1e-14) {                    // vertex is a minimum only when c > 0
    const tv = -d/(2*c);
    if (tv > 1e-9 && tv < t - 1e-9) cuts.push(tv);
  }
  cuts.push(t);
  return cuts;
}

function subseg(a, b, c, d, t0, t1) {
  const dt = t1 - t0, e = (c*t0 + d)*t0 + 1, s = 1/e, ps = dt*s*Math.sqrt(s);
  return [a*dt*ps, (b + a*t0)*ps, c*dt*dt*s, (d + 2*c*t0)*dt*s];
}
/*
 * Position integral z(t) = integral_0^t e^(i*theta) ds. GL-32 per cut
 * interval; intervals not starting at 0 are re-expressed via subseg so
 * each gets a fresh, well-conditioned chart.
 */
function integrate(a, b, c, d, t, theta) {
  theta = theta || makeTheta(a, b, c, d);
  const pts = [0, ...cutPoints(a, b, c, d, t), t];
  let zx = 0, zy = 0;
  for (let i = 0; i + 1 < pts.length; i++) {
    const w0 = pts[i], w1 = pts[i + 1];
    if (w1 - w0 < 1e-15) continue;
    if (w0 === 0) {
      let sx = 0, sy = 0;
      for (let k = 0; k < 32; k++) {
        const u = 0.5*w1*(GL.x[k] + 1), th = theta(u);
        sx += GL.w[k]*Math.cos(th); sy += GL.w[k]*Math.sin(th);
      }
      zx += 0.5*w1*sx; zy += 0.5*w1*sy;
    } else {
      const sp = subseg(a, b, c, d, w0, w1), thS = makeTheta(...sp), off = theta(w0);
      let sx = 0, sy = 0;
      for (let k = 0; k < 32; k++) {
        const u = 0.5*(GL.x[k] + 1), th = off + thS(u);
        sx += GL.w[k]*Math.cos(th); sy += GL.w[k]*Math.sin(th);
      }
      zx += (w1 - w0)*0.5*sx; zy += (w1 - w0)*0.5*sy;
    }
  }
  return [zx, zy];
}
/*
 * Dense polyline of the curve normalized to endpoints (0,0)-(1,0):
 * incremental midpoint quadrature on a uniform grid plus clusters near
 * the cuts, then one similarity transform dividing by the exact endpoint
 * integral I1 (so endpoints land exactly).
 */
function sampleCurve(a, b, c, d, N) {
  const theta = makeTheta(a, b, c, d);
  let ts = [];
  for (let i = 0; i <= 256; i++) ts.push(i/256);
  for (const w of cutPoints(a, b, c, d, 1))
    for (let k = -20; k <= 20; k++) { const t = w + k*1e-4; if (t > 0 && t < 1) ts.push(t); }
  ts = [...new Set(ts)].sort((p, q) => p - q);
  const zs = [[0, 0]];
  for (let i = 1; i < ts.length; i++) {
    const t0 = ts[i-1], t1 = ts[i], th = theta(0.5*(t0 + t1));
    zs.push([zs[i-1][0] + (t1 - t0)*Math.cos(th), zs[i-1][1] + (t1 - t0)*Math.sin(th)]);
  }
  const I1 = integrate(a, b, c, d, 1, theta);
  const zE = zs[zs.length - 1], den = zE[0]*zE[0] + zE[1]*zE[1];
  const fx = (I1[0]*zE[0] + I1[1]*zE[1])/den, fy = (I1[1]*zE[0] - I1[0]*zE[1])/den;
  const out = [], n2 = I1[0]*I1[0] + I1[1]*I1[1];
  for (let i = 0; i < ts.length; i++) {
    const zx = zs[i][0]*fx - zs[i][1]*fy, zy = zs[i][0]*fy + zs[i][1]*fx;
    out.push([ts[i], (zx*I1[0] + zy*I1[1])/n2, (zy*I1[0] - zx*I1[1])/n2]);
  }
  return { pts: out, I1, theta };
}

/*
 * Baseline arm length: the "parabola rule" (see raphlinus.github.io,
 * "Cleaner parallel curves with Euler spirals", 2021) -- the arm at which
 * the cubic's midpoint coincides with the circular arc's apex, exactly,
 * at every angle. Arms are measured as u = l/l*, so u = 1 means
 * "circle-like arms".
 */
const lstar = th => 2/(3*Math.max(1e-3, 1 + Math.cos(th)));
/*
 * Map constants. K: baseline slope of ln q in ln u -- the family's
 * measured small-angle response (~2.2). CK: corner steepening; larger
 * values reach corner-like shapes at shorter arms (aesthetic dial).
 */
let K = 2.2;
let CK = 1;
/*
 * Given the denominator quadratic (c,d), solve (a,b) so the curve interpolates the
 * endpoint tangents (G1). theta is linear in (a,b), so the net turn
 * theta(1) = a*F(1) + b*G(1) = th1 - th0 eliminates b exactly -- and
 * pins the winding class, excluding stray full loops. What remains is a
 * scalar Newton on a: the residual is the start-tangent error, read off
 * the argument of the position integral I1.
 *
 * Init 1 uses the ghost cubic's midpoint tangent as an estimate of
 * theta(1/2); with the net-turn condition that is a closed-form 2x2
 * (again by linearity). Init 2 is the small-angle/Euler formula. A
 * converged solution whose interior theta excursion leaves the principal
 * band (a hidden +-2pi round trip) is rejected and the next init tried.
 */
function fitAB(c, d, th0, th1, l0, l1) {
  const thF = makeTheta(1, 0, c, d), thG = makeTheta(0, 1, c, d);
  const F1 = thF(1), G1 = thG(1), dth = th1 - th0;
  if (Math.abs(G1) < 1e-30) return null;
  const bOf = a => (dth - a*F1)/G1;
  const r1 = a => {
    const b = bOf(a);
    const theta = t => a*thF(t) + b*thG(t);
    const I1 = integrate(a, b, c, d, 1, theta);
    if (cAbs(I1) < 1e-12) return null;
    return wrapA(-Math.atan2(I1[1], I1[0]) - th0);
  };
  // init ladder: midpoint-tangent informed (closed-form 2x2), then Euler
  const inits = [];
  {
    const dx = 2 - l0*Math.cos(th0) - l1*Math.cos(th1);
    const dy = -l0*Math.sin(th0) - l1*Math.sin(th1);
    const raw = (Math.hypot(dx, dy) > 1e-9) ? Math.atan2(-dy, dx) : th0 + 0.5*dth;
    const estg = 0.5*dth + wrapA(raw - th0 - 0.5*dth);
    const Fh = thF(0.5), Gh = thG(0.5);
    const det = Fh*G1 - Gh*F1;
    if (Math.abs(det) > 1e-10*Math.max(Math.abs(Fh*G1), Math.abs(Gh*F1), 1e-300))
      inits.push((estg*G1 - Gh*dth)/det);
  }
  inits.push(6*(th0 + th1));
  for (let ii = 0; ii < inits.length; ii++) {
    const a0 = inits[ii];
    let a = a0, r = r1(a);
    if (r === null) continue;
    for (let it = 0; it < 30; it++) {
      if (Math.abs(r) < 1e-11) break;
      const h = 1e-7*Math.max(1, Math.abs(a));
      const rp = r1(a + h);
      if (rp === null) { r = 1; break; }
      const dr = (rp - r)/h;
      if (dr === 0) { r = 1; break; }
      let step = -r/dr, lam = 1, ok = false;
      for (let ls = 0; ls < 12; ls++) {
        const rn = r1(a + lam*step);
        if (rn !== null && Math.abs(rn) < Math.abs(r)) { a += lam*step; r = rn; ok = true; break; }
        lam *= 0.5;
      }
      if (!ok) break;
    }
    if (Math.abs(r) >= 1e-8) continue;
    const b = bOf(a);
    // reject interior-excursion loopers (net turn right, round trip inside)
    if (Math.abs(a) > 1e-14) {
      const tx = -b/a;
      if (tx > 0 && tx < 1) {
        const ex = a*thF(tx) + b*thG(tx);
        if (!(Math.min(0, dth) - Math.PI - 0.3 < ex && ex < Math.max(0, dth) + Math.PI + 0.3))
          continue;
      }
    }
    return [a, b, ii === 0 ? 'mid' : 'euler-fallback'];
  }
  return null;
}
/*
 * The control-polygon -> hyperbezier map.
 *
 *   u, v : arms normalized by l*. Written as l*1.5*(1+cos th) = l/l*
 *          with a guard so u -> 0 continuously as a tangent approaches
 *          +-180 deg. That fade is what makes rotating a control point
 *          through the back direction continuous: the +-2pi winding
 *          ambiguity collapses into a sub-pixel curl at the endpoint.
 *
 *   ln q = K*ln u + CK*max(ln u, 0)^3, then sc(), a smooth clamp
 *          (p-norm min with radius L): 5.75..8 above (corner headroom),
 *          15 below (deep enough that the residual seam at +-180 deg is
 *          ~3e-5 of the chord). Note sc(0) = 0, so u = v = 1 maps to
 *          q0 = q1 = 1: an exact Euler spiral or circle at every angle.
 *
 *   qm   : this representation fixes the gauge freedom by normalizing
 *          integral_0^1 q^(-3/2) ds = 1 (rather than q(0) = 1), with
 *          (q0, qm, q1) the Bernstein weights of q. The antiderivative
 *          of q^(-3/2) is elementary and the condition telescopes to a
 *          closed form for qm.
 *
 *   Then convert to the q(0) = 1 gauge used above and solve the angles.
 */
function solveMap(th0, th1, l0, l1) {
  const t0 = performance.now();
  const u = l0*1.5*Math.max(1 + Math.cos(th0), 1e-12);
  const v = l1*1.5*Math.max(1 + Math.cos(th1), 1e-12);
  const LHI = Math.min(8, 5 + 0.75*CK);
  const sc = x => {
    x = Math.max(x, -60);
    const L = x >= 0 ? LHI : 15;
    return x/Math.pow(1 + Math.pow(x/L, 4), 0.25);
  };
  const lu = Math.log(Math.max(u, 1e-26)), lv = Math.log(Math.max(v, 1e-26));
  const q0 = Math.exp(sc(K*lu + CK*Math.pow(Math.max(lu, 0), 3)));
  const q1 = Math.exp(sc(K*lv + CK*Math.pow(Math.max(lv, 0), 3)));
  const qm = 1/Math.sqrt(q0) + 1/Math.sqrt(q1) - Math.sqrt(q0*q1);
  const c = (q0 - 2*qm + q1)/q0, d = 2*(qm - q0)/q0;
  const ab = fitAB(c, d, th0, th1, l0, l1);
  if (!ab) return null;
  const clamped = Math.abs(Math.log(u)) > 1.4 || Math.abs(Math.log(v)) > 1.4;
  return { x: [ab[0], ab[1], c, d], q0, q1, qm, u, v, clamped, init: ab[2],
           ms: performance.now() - t0 };
}
/* ---------- UI ---------- */
const cv = document.getElementById('cv'), ctx = cv.getContext('2d');
const CW = 730, CH = 500;
(function hidpi() {
  const dpr = window.devicePixelRatio || 1;
  cv.style.width = CW + 'px'; cv.style.height = CH + 'px';
  cv.width = Math.round(CW*dpr); cv.height = Math.round(CH*dpr);
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
})();
let P = [[180, 430], [300, 240], [520, 240], [640, 430]];
let dragging = -1, lastSol = null, stalled = false;

function preset(name) {
  const p0 = [120, 350], p3 = [610, 350], ch = 460;
  const set = (th0, th1, l0, l1) => {
    P = [p0, [p0[0] + l0*ch*Math.cos(th0), p0[1] - l0*ch*Math.sin(th0)],
         [p3[0] - l1*ch*Math.cos(th1), p3[1] + l1*ch*Math.sin(th1)], p3];
  };
  const d = Math.PI/180, D = Math.SQRT1_2;
  if (name === 'circle') set(45*d, -45*d, lstar(45*d), lstar(45*d));
  for (const th of [60, 90, 120, 135])
    if (name === 'e' + th) set(th*d, -th*d, lstar(th*d), lstar(th*d));
  if (name === 's') set(30*d, 30*d, lstar(30*d), lstar(30*d));
  if (name === 'corner') set(45*d, -45*d, 1.6*lstar(45*d), 1.6*lstar(45*d));
  if (name === 'past') set(45*d, -45*d, 2.4*lstar(45*d), 2.4*lstar(45*d));
  render();
}
function polygonData() {
  const dx = P[3][0] - P[0][0], dy = -(P[3][1] - P[0][1]);
  const ch = Math.hypot(dx, dy), chA = Math.atan2(dy, dx);
  const a0 = Math.atan2(-(P[1][1] - P[0][1]), P[1][0] - P[0][0]);
  const a1 = Math.atan2(-(P[3][1] - P[2][1]), P[3][0] - P[2][0]);
  return { ch, chA, th0: wrapA(a0 - chA), th1: wrapA(a1 - chA),
           l0: Math.hypot(P[1][0]-P[0][0], P[1][1]-P[0][1])/ch,
           l1: Math.hypot(P[3][0]-P[2][0], P[3][1]-P[2][1])/ch };
}
function toCanvas(zx, zy, g) {
  const ca = Math.cos(g.chA), sa = Math.sin(g.chA);
  const wx = (zx*ca - zy*sa)*g.ch, wy = (zx*sa + zy*ca)*g.ch;
  return [P[0][0] + wx, P[0][1] - wy];
}
function render() {
  const g = polygonData();
  ctx.clearRect(0, 0, CW, CH);
  ctx.strokeStyle = '#99b'; ctx.setLineDash([4, 4]); ctx.lineWidth = 1;
  ctx.beginPath(); ctx.moveTo(...P[0]); for (let i = 1; i < 4; i++) ctx.lineTo(...P[i]); ctx.stroke();
  ctx.setLineDash([]);
  if (document.getElementById('showCubic').checked) {
    ctx.strokeStyle = '#bbb'; ctx.lineWidth = 5; ctx.globalAlpha = 0.55;
    ctx.beginPath(); ctx.moveTo(...P[0]);
    ctx.bezierCurveTo(P[1][0], P[1][1], P[2][0], P[2][1], P[3][0], P[3][1]);
    ctx.stroke(); ctx.globalAlpha = 1;
  }
  const sol = solveMap(g.th0, g.th1, g.l0, g.l1);
  stalled = !sol;
  const use = sol || lastSol;
  if (use) {
    if (sol) lastSol = sol;
    const [a, b, c, d] = use.x;
    const S = sampleCurve(a, b, c, d, 300);
    ctx.strokeStyle = stalled ? '#c88' : '#c22'; ctx.lineWidth = 2;
    ctx.beginPath();
    S.pts.forEach((p, i) => { const q = toCanvas(p[1], p[2], g); i ? ctx.lineTo(...q) : ctx.moveTo(...q); });
    ctx.stroke();
    if (document.getElementById('showComb').checked) {
      const I1c = integrate(a, b, c, d, 1);
      const I1n = Math.hypot(...I1c), phiC = Math.atan2(I1c[1], I1c[0]);
      ctx.strokeStyle = '#2a7'; ctx.lineWidth = 0.7;
      for (let i = 0; i < S.pts.length; i += 6) {
        const [t, zx, zy] = S.pts[i], q = (c*t + d)*t + 1;
        const kap = (a*t + b)/(q*Math.sqrt(q))*I1n;
        const th = S.theta(t) - phiC;
        const nx = -Math.sin(th), ny = Math.cos(th);
        const L = Math.max(-0.25, Math.min(0.25, -0.05*kap));
        const p1 = toCanvas(zx, zy, g), p2 = toCanvas(zx + L*nx, zy + L*ny, g);
        ctx.beginPath(); ctx.moveTo(...p1); ctx.lineTo(...p2); ctx.stroke();
      }
    }
  }
  const names = ['P0', 'C1', 'C2', 'P3'];
  P.forEach((p, i) => {
    ctx.fillStyle = i === 0 || i === 3 ? '#036' : '#07c';
    ctx.beginPath(); ctx.arc(p[0], p[1], 6, 0, 7); ctx.fill();
    ctx.fillStyle = '#000'; ctx.fillText(names[i], p[0] + 9, p[1] - 6);
  });
  const dd = 180/Math.PI;
  let txt = `th0 ${(g.th0*dd).toFixed(1)}°  th1 ${(g.th1*dd).toFixed(1)}°\n`
    + `arms ${g.l0.toFixed(3)}, ${g.l1.toFixed(3)}\n`;
  if (use) {
    const T = use.T;
    const [a, b, c, d] = use.x;
    const tv = c > 0 ? -0.5*d/c : 0.5;
    const qm = (tv > 0 && tv < 1 && c > 0) ? (c*tv + d)*tv + 1 : Math.min(1, 1 + c + d);
    const eul = Math.abs(Math.log(use.q0)) + Math.abs(Math.log(use.q1));
    txt += `u ${use.u.toFixed(3)}  v ${use.v.toFixed(3)}`
      + (use.clamped ? `  [far field]` : ``) + `\n`
      + `q0 ${use.q0.toFixed(4)}  q1 ${use.q1.toFixed(4)}  qm ${use.qm.toFixed(4)}\n`
      + `q_min ${qm.toExponential(2)}   init: ${use.init}   ${use.ms.toFixed(1)} ms\n`
      + (eul < 0.02 ? `q ≈ const: EULER FAMILY (exact spiral/circle)` : ``)
      + (stalled ? `\nSTALLED — holding last shape` : ``);
  } else txt += `no solution yet (drag from a preset)`;
  const status = document.getElementById('status');
  if (status) {
    status.textContent = txt;
  }
}
cv.addEventListener('pointerdown', e => {
  const r = cv.getBoundingClientRect(), x = e.clientX - r.left, y = e.clientY - r.top;
  let best = -1, bd = 200;
  P.forEach((p, i) => { const d = (p[0]-x)**2 + (p[1]-y)**2; if (d < bd) { bd = d; best = i; } });
  dragging = best;
  cv.setPointerCapture(e.pointerId);
});
cv.addEventListener('pointermove', e => {
  if (dragging < 0) return;
  const r = cv.getBoundingClientRect();
  const nx = e.clientX - r.left, ny = e.clientY - r.top;
  if (dragging === 0 || dragging === 3) {
    const h = dragging === 0 ? 1 : 2;
    P[h][0] += nx - P[dragging][0]; P[h][1] += ny - P[dragging][1];
  }
  P[dragging] = [nx, ny];
  render();
});
cv.addEventListener('pointerup', () => { dragging = -1; render(); });
document.querySelectorAll('input').forEach(el => el.addEventListener('change', render));
/*
document.getElementById('ckslider').addEventListener('input', e => {
  CK = parseFloat(e.target.value);
  document.getElementById('ckval').textContent = CK.toFixed(2);
  render();
});
document.getElementById('kslider').addEventListener('input', e => {
  K = parseFloat(e.target.value);
  document.getElementById('kval').textContent = K.toFixed(2);
  render();
});
*/
preset('circle');
