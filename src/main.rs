use std::fmt;

// --- 1. Data Structures ---

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

// Custom Debug for cleaner printing (e.g., "(10.5, 20.0)")
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.1}, {:.1})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    x_min: f64,
    y_min: f64,
    x_max: f64,
    y_max: f64,
}

#[derive(Debug, Clone, Copy)]
struct Line {
    p1: Point,
    p2: Point,
}

// --- 2. Region Code Constants ---
// These are bit flags. A u8 is more than enough.
const INSIDE: u8 = 0b0000;  // 0
const LEFT: u8 = 0b0001;    // 1
const RIGHT: u8 = 0b0010;   // 2
const BOTTOM: u8 = 0b0100;  // 4
const TOP: u8 = 0b1000;     // 8


// --- 3. Outcode Computation Function ---

/// Computes the 4-bit "outcode" for a given point relative to the window.
fn compute_outcode(p: Point, window: &Rectangle) -> u8 {
    let mut code = INSIDE;

    if p.x < window.x_min {
        code |= LEFT;
    } else if p.x > window.x_max {
        code |= RIGHT;
    }

    if p.y < window.y_min {
        code |= BOTTOM;
    } else if p.y > window.y_max {
        code |= TOP;
    }

    code
}

// --- 4. The Main Clipping Algorithm ---

/// Clips a line to a rectangular window using the Cohen-Sutherland algorithm.
/// Returns Some(Line) if any part of the line is visible, None otherwise.
fn cohen_sutherland_clip(mut line: Line, window: &Rectangle) -> Option<Line> {
    // Compute outcodes for both endpoints
    let mut outcode1 = compute_outcode(line.p1, window);
    let mut outcode2 = compute_outcode(line.p2, window);

    loop {
        if (outcode1 | outcode2) == INSIDE {
            // --- Trivial Accept ---
            // Both endpoints are inside the window.
            return Some(line);
        } else if (outcode1 & outcode2) != INSIDE {
            // --- Trivial Reject ---
            // Both endpoints share an outside region (e.g., both are
            // to the LEFT, or both are TOP-LEFT). The line
            // cannot possibly cross the window.
            return None;
        } else {
            // --- Potential Clip ---
            // The line needs to be clipped. We'll clip one of the
            // endpoints that is outside the window.

            // First, pick an endpoint that is outside.
            // If outcode1 is outside, use it; otherwise, use outcode2.
            let outcode_to_clip = if outcode1 != INSIDE { outcode1 } else { outcode2 };

            let mut new_p = Point { x: 0.0, y: 0.0 };
            let dx = line.p2.x - line.p1.x;
            let dy = line.p2.y - line.p1.y;

            // Find the intersection point using line-boundary intersections.
            // This uses the parametric form of a line:
            // x = x1 + dx * t
            // y = y1 + dy * t
            // We find the 't' value at the boundary and calculate the
            // corresponding x or y.
            //
            // A more direct (and common) way is to use slope-intercept:
            // y = y1 + slope * (x - x1)  (where slope = dy / dx)
            // x = x1 + (y - y1) / slope  (where 1/slope = dx / dy)

            if (outcode_to_clip & TOP) != 0 {
                // Point is above, clip to top boundary
                new_p.x = line.p1.x + dx * (window.y_max - line.p1.y) / dy;
                new_p.y = window.y_max;
            } else if (outcode_to_clip & BOTTOM) != 0 {
                // Point is below, clip to bottom boundary
                new_p.x = line.p1.x + dx * (window.y_min - line.p1.y) / dy;
                new_p.y = window.y_min;
            } else if (outcode_to_clip & RIGHT) != 0 {
                // Point is right, clip to right boundary
                new_p.y = line.p1.y + dy * (window.x_max - line.p1.x) / dx;
                new_p.x = window.x_max;
            } else if (outcode_to_clip & LEFT) != 0 {
                // Point is left, clip to left boundary
                new_p.y = line.p1.y + dy * (window.x_min - line.p1.x) / dx;
                new_p.x = window.x_min;
            }

            // Now, replace the outside point with the new intersection point
            if outcode_to_clip == outcode1 {
                line.p1 = new_p;
                outcode1 = compute_outcode(line.p1, window);
            } else {
                line.p2 = new_p;
                outcode2 = compute_outcode(line.p2, window);
            }
        }
        // The loop continues with the new, shorter line segment.
    }
}

// --- 5. Main Function with Test Cases ---

fn main() {
    // Define a 100x100 clipping window
    let window = Rectangle {
        x_min: 100.0,
        y_min: 100.0,
        x_max: 200.0,
        y_max: 200.0,
    };
    println!("--- Clipping Window: {:?} ---", window);

    // Case 1: Trivial Accept (Line fully inside)
    let line1 = Line {
        p1: Point { x: 110.0, y: 110.0 },
        p2: Point { x: 190.0, y: 190.0 },
    };
    println!("\nTest 1 (Accept):  {:?}", line1);
    println!("Result:         {:?}", cohen_sutherland_clip(line1, &window));

    // Case 2: Trivial Reject (Line fully outside, to the right)
    let line2 = Line {
        p1: Point { x: 210.0, y: 110.0 },
        p2: Point { x: 250.0, y: 190.0 },
    };
    println!("\nTest 2 (Reject):  {:?}", line2);
    println!("Result:         {:?}", cohen_sutherland_clip(line2, &window));

    // Case 3: Trivial Reject (Line fully outside, top-left to top-right)
    let line3 = Line {
        p1: Point { x: 50.0, y: 250.0 },
        p2: Point { x: 250.0, y: 250.0 },
    };
    println!("\nTest 3 (Reject):  {:?}", line3);
    println!("Result:         {:?}", cohen_sutherland_clip(line3, &window));


    // Case 4: Clipping (Diagonal line crossing two corners)
    let line4 = Line {
        p1: Point { x: 50.0, y: 50.0 },
        p2: Point { x: 250.0, y: 250.0 },
    };
    println!("\nTest 4 (Clip 2-Corners): {:?}", line4);
    println!("Result:              {:?}", cohen_sutherland_clip(line4, &window));
    // Expected: Some(Line { p1: (100.0, 100.0), p2: (200.0, 200.0) })

    // Case 5: Clipping (Horizontal line crossing left and right)
    let line5 = Line {
        p1: Point { x: 50.0, y: 150.0 },
        p2: Point { x: 250.0, y: 150.0 },
    };
    println!("\nTest 5 (Clip L-R):  {:?}", line5);
    println!("Result:           {:?}", cohen_sutherland_clip(line5, &window));
    // Expected: Some(Line { p1: (100.0, 150.0), p2: (200.0, 150.0) })

    // Case 6: Clipping (Vertical line crossing top and bottom)
    let line6 = Line {
        p1: Point { x: 150.0, y: 50.0 },
        p2: Point { x: 150.0, y: 250.0 },
    };
    println!("\nTest 6 (Clip T-B):  {:?}", line6);
    println!("Result:           {:?}", cohen_sutherland_clip(line6, &window));
    // Expected: Some(Line { p1: (150.0, 100.0), p2: (150.0, 200.0) })

    // Case 7: Clipping (One point inside, one outside)
    let line7 = Line {
        p1: Point { x: 150.0, y: 150.0 }, // Inside
        p2: Point { x: 250.0, y: 250.0 }, // Outside (Top-Right)
    };
    println!("\nTest 7 (Clip 1-End): {:?}", line7);
    println!("Result:            {:?}", cohen_sutherland_clip(line7, &window));
    // Expected: Some(Line { p1: (150.0, 150.0), p2: (200.0, 200.0) })
}
