///////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2021 David Thompson
//
// All rights reserved. This program and the accompanying materials
// are made available under the terms of the Eclipse Public License v2.0
// which accompanies this distribution, and is available at
// http://www.eclipse.org/legal/epl-v20.html
//
// SPDX-License-Identifier: EPL-2.0
///////////////////////////////////////////////////////////////////////////////
use pancurses::{
    curs_set, endwin, init_pair, initscr, napms, start_color, use_default_colors, Attributes,
    ColorPair, COLORS, COLOR_BLUE,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    // Init logic
    let mut x = 1;
    let mut y = 0;
    let mut x_vel = 1;
    let mut y_vel = 1;

    // Make window
    let window = initscr();

    // Set up interrupt handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Colors & bold
    start_color();
    if COLORS() >= 8 {
        use_default_colors();
        // -1 is the default background
        init_pair(1, COLOR_BLUE, -1);
        let mut attr = Attributes::new();
        attr.set_bold(true);
        attr.set_color_pair(ColorPair(1));
        window.attrset(attr);
    }

    // Don't show the cursor
    curs_set(0);

    while running.load(Ordering::SeqCst) {
        // Logo bounce logic
        let max_x = window.get_max_x();
        let max_y = window.get_max_y();
        x += x_vel;
        y += y_vel;
        x = if x + 2 >= max_x {
            x_vel = -1;
            max_x - 4
        } else if x < 0 {
            x_vel = 1;
            1
        } else {
            x
        };
        y = if y >= max_y {
            y_vel = -1;
            max_y - 2
        } else if y < 0 {
            y_vel = 1;
            1
        } else {
            y
        };

        // Drawing
        window.clear();
        window.mv(y, x);
        window.addstr("DVD");
        window.refresh();
        napms(1000);
    }
    endwin();
}
